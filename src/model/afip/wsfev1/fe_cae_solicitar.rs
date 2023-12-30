use std::str::FromStr;

use chrono::{Local, Utc, NaiveDateTime};
use reqwest::{Client, Error};
use rocket_db_pools::Connection;
use yaserde_derive::{YaSerialize,YaDeserialize};
use yaserde;
use crate::{Db, CONF};
use crate::aux_func::time_serde::date_to_yyyymmdd;
use crate::model::afip::soap_utils::get_xml_tag;
use crate::model::propio::factura::{db_factura_transmision, OPERACION::ENVIO, OPERACION::RESPUESTA};
use crate::{model::afip::soap_utils::{afip_post, afip_signin}, types::Factura};

use rand::Rng;
use super::constants::*;

/** Ingresa a afip, registra el intento de transmision y luego transmite a afip
 * Modifica la factura, agregandole cae y venc_cae
 */
pub async fn afip_fe_cae_solicitar(
	req_cli: &Client,
	factura: &mut Factura,
	db: &mut Connection<Db>,
	mut err_msg: &String
) -> Result<bool, Error>{
	let id_transmision = rand::thread_rng().gen_range(1..=100000);
	let url = if CONF.is_prd() {WSFEV1_PRD} else {WSFEV1_VAL};

	let auth = afip_signin(req_cli, "WSFE").await;
	let soap_msg = factura_to_soap(factura,auth);
	let _ = db_factura_transmision(db, factura.id_factura,id_transmision, ENVIO, &soap_msg).await;

	let respuesta = afip_post(req_cli, url,soap_msg).await?;
	let _ = db_factura_transmision(db, factura.id_factura,id_transmision, RESPUESTA, &respuesta).await;

	let body = get_xml_tag(&respuesta, "soap:Body");
	let parsed:Result<FecaesolicitarResponse, ()> = 
		yaserde::de::from_str(&body).map_err(|e| {
			println!("Deserialization error: {:?}", e);
		});

	match parsed {
		Ok(a) => {
			println!("parsed ok:{:?}",a);
			let result = a.fecae_solicitar_result.unwrap();

			match result.errors {
				Some(afip_err) => {
					println!("afip_err:{:?}",afip_err);
					err_msg = &mut format!("afip_err:{:?}",afip_err);
					return Ok(false);
				}
				None => {
					let d2 = result.fe_det_resp.unwrap();
					let d3 = d2.fecae_det_response.get(0).unwrap();
					let venci_str = d3.cae_fch_vto.as_ref().unwrap();
					let venci = NaiveDateTime::parse_from_str(&venci_str,"%Y%m%d").unwrap().and_local_timezone(Local).single().unwrap();
					factura.cae = d3.cae.as_ref().unwrap().parse().unwrap();
					factura.venc_cae = venci.with_timezone(&Utc);
				}

			}			
		}
		Err(e) => {
			err_msg = &mut format!("parsed err:{:?}",e);
			println!("parsed err:{:?}",e);
			return Ok(false);
		}

	}

	return Ok(true);
}

fn factura_to_soap(
	factura: &Factura, auth:String
) -> String {
	let body = Fecaesolicitar {
    auth,
    fe_cae_req: Some(Fecaerequest{
        fe_cab_req: Some(FecaecabRequest{
            fecab_request: FecabRequest {
                cant_reg: 	1,
                pto_vta: 		factura.punto_venta,
                cbte_tipo: 	factura.tipo_fac,
            }
        }),
        fe_det_req: Some(ArrayOfFECAEDetRequest{
            fecae_det_request: vec![
							FecaedetRequest{ 
								fedet_request: FedetRequest{ 
									concepto: 			1, //1 Productos, 2 Servicios, 3 Ambos
									doc_tipo: 			factura.cliente.tipo_doc,  //Código de documento identificatorio del comprador
									doc_nro: 				factura.cliente.num_doc, 
									cbte_desde: 		factura.numero,
									cbte_hasta: 		factura.numero,
									cbte_fch: 			Some(date_to_yyyymmdd(factura.fecha)), 
									imp_total: 			factura.total(),
									imp_tot_conc: 	0.0, //para tipo C tiene que ser cero
									imp_neto:	 			factura.total(), //para tipo C, este es el subtotal
									imp_op_ex: 			0.0, //para tipo C tiene que ser cero
									imp_trib: 			0.0,
									imp_iva: 				0.0, //para tipo C tiene que ser cero
									fch_serv_desde: None, //Solo se informa si concepto NO es producto
									fch_serv_hasta: None, //Solo se informa si concepto NO es producto
									fch_vto_pago: 	None, //Solo se informa si concepto NO es producto
									mon_id: 				Some(String::from("PES")), 
									mon_cotiz: 			1.0, 
									cbtes_asoc: 		None, 
									tributos: 			None, 
									iva: 						None, 
									opcionales: 		None,
									compradores: 		None, 
									periodo_asoc: 	None, 
									actividades: 		None
								}, 
								xsi_type: String::from("No tengo idea que es esto") }
						]
						,
        }),
    }),

	};

	return format!(
		"<soap:Envelope 
			xmlns:soap=\"http://www.w3.org/2003/05/soap-envelope\" 
			xmlns:ar=\"http://ar.gov.afip.dif.FEV1/\">
			<soap:Header/>
			<soap:Body>
				{}
			</soap:Body>
			</soap:Envelope>",
			yaserde::ser::to_string(&body).expect("Failed to serialize to XML")
		);
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitar",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
struct Fecaesolicitar {
	#[yaserde(rename = "Auth", prefix = "tns", default)]
	pub auth: String, 
	#[yaserde(rename = "FeCAEReq", prefix = "tns", default)]
	pub fe_cae_req: Option<Fecaerequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEAuthRequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FeauthRequest {
	#[yaserde(rename = "Token", prefix = "tns", default)]
	pub token: Option<String>, 
	#[yaserde(rename = "Sign", prefix = "tns", default)]
	pub sign: Option<String>, 
	#[yaserde(rename = "Cuit", prefix = "tns", default)]
	pub cuit: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAERequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Fecaerequest {
	#[yaserde(rename = "FeCabReq", prefix = "tns", default)]
	pub fe_cab_req: Option<FecaecabRequest>, 
	#[yaserde(rename = "FeDetReq", prefix = "tns", default)]
	pub fe_det_req: Option<ArrayOfFECAEDetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAECabRequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FecaecabRequest {
	#[yaserde(flatten, default)]
	pub fecab_request: FecabRequest
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECabRequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FecabRequest {
	#[yaserde(rename = "CantReg", prefix = "tns", default)]
	pub cant_reg: i32, 
	#[yaserde(rename = "PtoVta", prefix = "tns", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "tns", default)]
	pub cbte_tipo: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEDetRequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfFECAEDetRequest {
	#[yaserde(rename = "FECAEDetRequest", prefix = "tns", default)]
	pub fecae_det_request: Vec<FecaedetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEDetRequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FecaedetRequest {
	#[yaserde(flatten, default)]
	pub fedet_request: FedetRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDetRequest",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FedetRequest {
	#[yaserde(rename = "Concepto", prefix = "tns", default)]
	pub concepto: i32, 
	#[yaserde(rename = "DocTipo", prefix = "tns", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "tns", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "CbteDesde", prefix = "tns", default)]
	pub cbte_desde: i64, 
	#[yaserde(rename = "CbteHasta", prefix = "tns", default)]
	pub cbte_hasta: i64, 
	#[yaserde(rename = "CbteFch", prefix = "tns", default)]
	pub cbte_fch: Option<String>, 
	#[yaserde(rename = "ImpTotal", prefix = "tns", default)]
	pub imp_total: f64, 
	#[yaserde(rename = "ImpTotConc", prefix = "tns", default)]
	pub imp_tot_conc: f64, 
	#[yaserde(rename = "ImpNeto", prefix = "tns", default)]
	pub imp_neto: f64, 
	#[yaserde(rename = "ImpOpEx", prefix = "tns", default)]
	pub imp_op_ex: f64, 
	#[yaserde(rename = "ImpTrib", prefix = "tns", default)]
	pub imp_trib: f64, 
	#[yaserde(rename = "ImpIVA", prefix = "tns", default)]
	pub imp_iva: f64, 
	#[yaserde(rename = "FchServDesde", prefix = "tns", default)]
	pub fch_serv_desde: Option<String>, 
	#[yaserde(rename = "FchServHasta", prefix = "tns", default)]
	pub fch_serv_hasta: Option<String>, 
	#[yaserde(rename = "FchVtoPago", prefix = "tns", default)]
	pub fch_vto_pago: Option<String>, 
	#[yaserde(rename = "MonId", prefix = "tns", default)]
	pub mon_id: Option<String>, 
	#[yaserde(rename = "MonCotiz", prefix = "tns", default)]
	pub mon_cotiz: f64, 
	#[yaserde(rename = "CbtesAsoc", prefix = "tns", default)]
	pub cbtes_asoc: Option<ArrayOfCbteAsoc>, 
	#[yaserde(rename = "Tributos", prefix = "tns", default)]
	pub tributos: Option<ArrayOfTributo>, 
	#[yaserde(rename = "Iva", prefix = "tns", default)]
	pub iva: Option<ArrayOfAlicIva>, 
	#[yaserde(rename = "Opcionales", prefix = "tns", default)]
	pub opcionales: Option<ArrayOfOpcional>, 
	#[yaserde(rename = "Compradores", prefix = "tns", default)]
	pub compradores: Option<ArrayOfComprador>, 
	#[yaserde(rename = "PeriodoAsoc", prefix = "tns", default)]
	pub periodo_asoc: Option<Periodo>, 
	#[yaserde(rename = "Actividades", prefix = "tns", default)]
	pub actividades: Option<ArrayOfActividad>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCbteAsoc",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfCbteAsoc {
	#[yaserde(rename = "CbteAsoc", prefix = "tns", default)]
	pub cbte_asoc: Vec<CbteAsoc>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CbteAsoc",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct CbteAsoc {
	#[yaserde(rename = "Tipo", prefix = "tns", default)]
	pub tipo: i32, 
	#[yaserde(rename = "PtoVta", prefix = "tns", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "Nro", prefix = "tns", default)]
	pub nro: i64, 
	#[yaserde(rename = "Cuit", prefix = "tns", default)]
	pub cuit: Option<String>, 
	#[yaserde(rename = "CbteFch", prefix = "tns", default)]
	pub cbte_fch: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTributo",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfTributo {
	#[yaserde(rename = "Tributo", prefix = "tns", default)]
	pub tributo: Vec<Tributo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Tributo",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Tributo {
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: i16, 
	#[yaserde(rename = "Desc", prefix = "tns", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "BaseImp", prefix = "tns", default)]
	pub base_imp: f64, 
	#[yaserde(rename = "Alic", prefix = "tns", default)]
	pub alic: f64, 
	#[yaserde(rename = "Importe", prefix = "tns", default)]
	pub importe: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAlicIva",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfAlicIva {
	#[yaserde(rename = "AlicIva", prefix = "tns", default)]
	pub alic_iva: Vec<AlicIva>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlicIva",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct AlicIva {
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: i32, 
	#[yaserde(rename = "BaseImp", prefix = "tns", default)]
	pub base_imp: f64, 
	#[yaserde(rename = "Importe", prefix = "tns", default)]
	pub importe: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfOpcional",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfOpcional {
	#[yaserde(rename = "Opcional", prefix = "tns", default)]
	pub opcional: Vec<Opcional>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Opcional",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Opcional {
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Valor", prefix = "tns", default)]
	pub valor: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfComprador",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfComprador {
	#[yaserde(rename = "Comprador", prefix = "tns", default)]
	pub comprador: Vec<Comprador>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Comprador",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Comprador {
	#[yaserde(rename = "DocTipo", prefix = "tns", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "tns", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "Porcentaje", prefix = "tns", default)]
	pub porcentaje: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Periodo",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Periodo {
	#[yaserde(rename = "FchDesde", prefix = "tns", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "tns", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfActividad",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfActividad {
	#[yaserde(rename = "Actividad", prefix = "tns", default)]
	pub actividad: Vec<Actividad>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Actividad",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Actividad {
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: i64, 
}

//**************************************************************************** */
//***************************Respuesta**************************************** */
//**************************************************************************** */

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitarResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
struct FecaesolicitarResponse {
	#[yaserde(rename = "FECAESolicitarResult", prefix = "tns", default)]
	pub fecae_solicitar_result: Option<Fecaeresponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Fecaeresponse {
	#[yaserde(rename = "FeCabResp", prefix = "tns", default)]
	pub fe_cab_resp: Option<FecaecabResponse>, 
	#[yaserde(rename = "FeDetResp", prefix = "tns", default)]
	pub fe_det_resp: Option<ArrayOfFECAEDetResponse>, 
	#[yaserde(rename = "Events", prefix = "tns", default)]
	pub events: Option<ArrayOfEvt>, 
	#[yaserde(rename = "Errors", prefix = "tns", default)]
	pub errors: Option<ArrayOfErr>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAECabResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FecaecabResponse {
	#[yaserde(flatten, default)]
	pub fecab_response: FecabResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECabResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FecabResponse {
	#[yaserde(rename = "Cuit", prefix = "tns", default)]
	pub cuit: i64, 
	#[yaserde(rename = "PtoVta", prefix = "tns", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "tns", default)]
	pub cbte_tipo: i32, 
	#[yaserde(rename = "FchProceso", prefix = "tns", default)]
	pub fch_proceso: Option<String>, 
	#[yaserde(rename = "CantReg", prefix = "tns", default)]
	pub cant_reg: i32, 
	#[yaserde(rename = "Resultado", prefix = "tns", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Reproceso", prefix = "tns", default)]
	pub reproceso: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEDetResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfFECAEDetResponse {
	#[yaserde(rename = "FECAEDetResponse", prefix = "tns", default)]
	pub fecae_det_response: Vec<FecaedetResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEDetResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FecaedetResponse {
	#[yaserde(flatten, default)]
	pub fedet_response: FedetResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "CAE", prefix = "tns", default)]
	pub cae: Option<String>, 
	#[yaserde(rename = "CAEFchVto", prefix = "tns", default)]
	pub cae_fch_vto: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDetResponse",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct FedetResponse {
	#[yaserde(rename = "Concepto", prefix = "tns", default)]
	pub concepto: i32, 
	#[yaserde(rename = "DocTipo", prefix = "tns", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "tns", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "CbteDesde", prefix = "tns", default)]
	pub cbte_desde: i64, 
	#[yaserde(rename = "CbteHasta", prefix = "tns", default)]
	pub cbte_hasta: i64, 
	#[yaserde(rename = "CbteFch", prefix = "tns", default)]
	pub cbte_fch: Option<String>, 
	#[yaserde(rename = "Resultado", prefix = "tns", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Observaciones", prefix = "tns", default)]
	pub observaciones: Option<ArrayOfObs>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfObs",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfObs {
	#[yaserde(rename = "Obs", prefix = "tns", default)]
	pub obs: Vec<Obs>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Obs",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Obs {
	#[yaserde(rename = "Code", prefix = "tns", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "tns", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEvt",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfEvt {
	#[yaserde(rename = "Evt", prefix = "tns", default)]
	pub evt: Vec<Evt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Evt",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Evt {
	#[yaserde(rename = "Code", prefix = "tns", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "tns", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfErr",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct ArrayOfErr {
	#[yaserde(rename = "Err", prefix = "tns", default)]
	pub err: Vec<Err>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Err",
	namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "tns",
)]
struct Err {
	#[yaserde(rename = "Code", prefix = "tns", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "tns", default)]
	pub msg: Option<String>, 
}