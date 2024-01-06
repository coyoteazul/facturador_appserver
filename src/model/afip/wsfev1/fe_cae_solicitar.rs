use chrono::{Utc, NaiveDate, Local};
use reqwest::{Client, Error};
use rocket_db_pools::Connection;
use yaserde_derive::{YaSerialize,YaDeserialize};
use yaserde;
use crate::model::afip::wsfev1::fe_comp_ultimo_autorizado;
use crate::model::propio::factura::db_factura_set_cae;
use crate::{Db, CONF};
use crate::aux_func::time_serde::date_to_yyyymmdd;
use crate::model::afip::soap_utils::get_xml_tag;
use crate::model::propio::factura::{db_factura_transmision, OPERACION::ENVIO, OPERACION::RESPUESTA};
use crate::{model::afip::soap_utils::{afip_post, afip_signin}, types::Factura};
use super::constants::*;

/** Ingresa a afip, registra el intento de transmision y luego transmite a afip
 * Modifica la factura, agregandole cae y venc_cae
 */
pub async fn afip_fe_cae_solicitar(
	req_cli: &Client,
	factura: &mut Factura,
	db: &mut Connection<Db>
) -> Result<(bool,Vec::<String>), Error>{
	dbg!("Function call");
	let url = if CONF.is_prd() {WSFEV1_PRD} else {WSFEV1_VAL};
	let mut es_ok = false; //Se cambia si sale ok
	let mut msg = Vec::<String>::new();
	

	match afip_signin(req_cli, "wsfe",db).await {
		Ok(auth) => {
			let soap_msg = factura_to_soap(factura,auth);
			let id_trans = db_factura_transmision(db, factura.id_factura,None, ENVIO, &soap_msg).await.unwrap();

			let respuesta = afip_post(req_cli, url,soap_msg).await?;
			let _ = db_factura_transmision(db, factura.id_factura,Some(id_trans), RESPUESTA, &respuesta).await;

			if respuesta.contains("<soap:Fault>") {
				msg.push(respuesta);
			} else {
				let body = get_xml_tag(&respuesta, "soap:Body")
				.replace("xmlns=\"http://ar.gov.afip.dif.FEV1/\"", "");

				let parsed:Result<FecaesolicitarResponse, String> = 
					yaserde::de::from_str(&body).map_err(|e| {
						return format!("Deserialization error: {:?}", e);
					});

				match parsed {
					Ok(a) => {
						dbg!("parsed ok:",&a,body);
						let result = a.fecae_solicitar_result.unwrap();

						match result.errors {
							Some(afip_err) => {
								for err in afip_err.err {
									match err.code {
										10016 => { //El numero de comprobante no se corresponde con el proximo a autorizar. Consultar metodo FECompUltimoAutorizado.
											msg.push("10016".to_string())
										}
										_ => {
											dbg!(&err.msg);
											msg.push(format!("{:?}",err.msg));
										}
									}
								}
								//return Ok((false,msg));
							}
							None => {
								dbg!("Tag de error");
							}
						}

						let d2 = result.fe_det_resp.unwrap();
						let d3: &FecaedetResponse = d2.fecae_det_response.get(0).unwrap();
						match &d3.resultado {
							Some(resultado) => {
								match resultado.as_str() {
									"A" => {
										es_ok = true;
										let venci_str = d3.cae_fch_vto.as_ref().unwrap();
										let venci = NaiveDate::parse_from_str(&venci_str, "%Y%m%d").unwrap().and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Local).single().unwrap().naive_utc().and_utc();
										factura.cae = d3.cae.as_ref().unwrap().parse().unwrap();
										factura.venc_cae = venci;
										let _ = db_factura_set_cae(db, factura).await;
									}
									"R" => {
										match &d3.observaciones {
											Some(arr_of_obs) => {
												for obs in &arr_of_obs.obs {
													match obs.code {
														10016 => { //El numero de comprobante no se corresponde con el proximo a autorizar. Consultar metodo FECompUltimoAutorizado.
															msg.push("10016".to_string())
														}
														_ => {
															msg.push(format!("{:?}",obs));
														}
													}
												}
											}
											None => {
												
											}
										}
									}
									_=> {
										let msg = "Se recibio un resultado que no es ni A ni R".to_string();
										dbg!(&msg);
									}
								}
							}
							None => {
								msg.push("Se recibio respuesta de afip pero no se encontro el tag 'resultado'".to_string());
								dbg!(&msg);							
							}
						}
					}
					Err(e) => {
						dbg!(&e);
						msg.push(e);
					}
				}	
			}		
		}
		Err(e) => {
			dbg!(&e);
			msg.push(e);
		}
	};
	
	return Ok((es_ok, msg));
}

fn factura_to_soap(
	factura: &Factura, auth:String
) -> String {
	dbg!("Function call");
	const SOAP_HEAD_SIZE:usize = "<?xml version=\"1.0\" encoding=\"utf-8\"?>".len();
	let body = Fecaerequest{
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
						}
					}
				]
				,
		})
	};

	let factura_str = yaserde::ser::to_string(&body)
	.expect("Failed to serialize to XML")
	//Remover cabezal
	.split_at(SOAP_HEAD_SIZE).1.to_string();


	return format!(r#"
	<soap:Envelope 
			xmlns:soap="http://www.w3.org/2003/05/soap-envelope" 
			xmlns:ar="http://ar.gov.afip.dif.FEV1/">
			<soap:Header/>
			<soap:Body>
			<ar:FECAESolicitar>
				{auth}
				{factura_str}
			</ar:FECAESolicitar>	
			</soap:Body>
			</soap:Envelope>"#
		);
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitar",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "ar",
)]
struct Fecaesolicitar {
	#[yaserde(rename = "Auth", prefix = "ar", default)]
	pub auth: String, 
	#[yaserde(rename = "FeCAEReq", prefix = "ar", default)]
	pub fe_cae_req: Option<Fecaerequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEAuthRequest",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct FeauthRequest {
	#[yaserde(rename = "Token", prefix = "ar", default)]
	pub token: Option<String>, 
	#[yaserde(rename = "Sign", prefix = "ar", default)]
	pub sign: Option<String>, 
	#[yaserde(rename = "Cuit", prefix = "ar", default)]
	pub cuit: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FeCAEReq",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct Fecaerequest {
	#[yaserde(rename = "FeCabReq", prefix = "ar", default)]
	pub fe_cab_req: Option<FecaecabRequest>, 
	#[yaserde(rename = "FeDetReq", prefix = "ar", default)]
	pub fe_det_req: Option<ArrayOfFECAEDetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAECabRequest",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct FecaecabRequest {
	#[yaserde(flatten, default)]
	pub fecab_request: FecabRequest
}

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECabRequest",
	prefix = "ar",
)]
struct FecabRequest {
	#[yaserde(rename = "CantReg", prefix = "ar", default)]
	pub cant_reg: i32, 
	#[yaserde(rename = "PtoVta", prefix = "ar", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "ar", default)]
	pub cbte_tipo: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEDetRequest",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfFECAEDetRequest {
	#[yaserde(rename = "FECAEDetRequest", prefix = "ar", default)]
	pub fecae_det_request: Vec<FecaedetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEDetRequest",
	prefix = "ar",
)]
struct FecaedetRequest {
	#[yaserde(flatten, default)]
	pub fedet_request: FedetRequest
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDetRequest",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct FedetRequest {
	#[yaserde(rename = "Concepto", prefix = "ar", default)]
	pub concepto: i32, 
	#[yaserde(rename = "DocTipo", prefix = "ar", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "ar", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "CbteDesde", prefix = "ar", default)]
	pub cbte_desde: i64, 
	#[yaserde(rename = "CbteHasta", prefix = "ar", default)]
	pub cbte_hasta: i64, 
	#[yaserde(rename = "CbteFch", prefix = "ar", default)]
	pub cbte_fch: Option<String>, 
	#[yaserde(rename = "ImpTotal", prefix = "ar", default)]
	pub imp_total: f64, 
	#[yaserde(rename = "ImpTotConc", prefix = "ar", default)]
	pub imp_tot_conc: f64, 
	#[yaserde(rename = "ImpNeto", prefix = "ar", default)]
	pub imp_neto: f64, 
	#[yaserde(rename = "ImpOpEx", prefix = "ar", default)]
	pub imp_op_ex: f64, 
	#[yaserde(rename = "ImpTrib", prefix = "ar", default)]
	pub imp_trib: f64, 
	#[yaserde(rename = "ImpIVA", prefix = "ar", default)]
	pub imp_iva: f64, 
	#[yaserde(rename = "FchServDesde", prefix = "ar", default)]
	pub fch_serv_desde: Option<String>, 
	#[yaserde(rename = "FchServHasta", prefix = "ar", default)]
	pub fch_serv_hasta: Option<String>, 
	#[yaserde(rename = "FchVtoPago", prefix = "ar", default)]
	pub fch_vto_pago: Option<String>, 
	#[yaserde(rename = "MonId", prefix = "ar", default)]
	pub mon_id: Option<String>, 
	#[yaserde(rename = "MonCotiz", prefix = "ar", default)]
	pub mon_cotiz: f64, 
	#[yaserde(rename = "CbtesAsoc", prefix = "ar", default)]
	pub cbtes_asoc: Option<ArrayOfCbteAsoc>, 
	#[yaserde(rename = "Tributos", prefix = "ar", default)]
	pub tributos: Option<ArrayOfTributo>, 
	#[yaserde(rename = "Iva", prefix = "ar", default)]
	pub iva: Option<ArrayOfAlicIva>, 
	#[yaserde(rename = "Opcionales", prefix = "ar", default)]
	pub opcionales: Option<ArrayOfOpcional>, 
	#[yaserde(rename = "Compradores", prefix = "ar", default)]
	pub compradores: Option<ArrayOfComprador>, 
	#[yaserde(rename = "PeriodoAsoc", prefix = "ar", default)]
	pub periodo_asoc: Option<Periodo>, 
	#[yaserde(rename = "Actividades", prefix = "ar", default)]
	pub actividades: Option<ArrayOfActividad>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCbteAsoc",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfCbteAsoc {
	#[yaserde(rename = "CbteAsoc", prefix = "ar", default)]
	pub cbte_asoc: Vec<CbteAsoc>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CbteAsoc",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct CbteAsoc {
	#[yaserde(rename = "Tipo", prefix = "ar", default)]
	pub tipo: i32, 
	#[yaserde(rename = "PtoVta", prefix = "ar", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "Nro", prefix = "ar", default)]
	pub nro: i64, 
	#[yaserde(rename = "Cuit", prefix = "ar", default)]
	pub cuit: Option<String>, 
	#[yaserde(rename = "CbteFch", prefix = "ar", default)]
	pub cbte_fch: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTributo",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfTributo {
	#[yaserde(rename = "Tributo", prefix = "ar", default)]
	pub tributo: Vec<Tributo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Tributo",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct Tributo {
	#[yaserde(rename = "Id", prefix = "ar", default)]
	pub id: i16, 
	#[yaserde(rename = "Desc", prefix = "ar", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "BaseImp", prefix = "ar", default)]
	pub base_imp: f64, 
	#[yaserde(rename = "Alic", prefix = "ar", default)]
	pub alic: f64, 
	#[yaserde(rename = "Importe", prefix = "ar", default)]
	pub importe: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAlicIva",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfAlicIva {
	#[yaserde(rename = "AlicIva", prefix = "ar", default)]
	pub alic_iva: Vec<AlicIva>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlicIva",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct AlicIva {
	#[yaserde(rename = "Id", prefix = "ar", default)]
	pub id: i32, 
	#[yaserde(rename = "BaseImp", prefix = "ar", default)]
	pub base_imp: f64, 
	#[yaserde(rename = "Importe", prefix = "ar", default)]
	pub importe: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfOpcional",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfOpcional {
	#[yaserde(rename = "Opcional", prefix = "ar", default)]
	pub opcional: Vec<Opcional>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Opcional",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct Opcional {
	#[yaserde(rename = "Id", prefix = "ar", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Valor", prefix = "ar", default)]
	pub valor: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfComprador",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfComprador {
	#[yaserde(rename = "Comprador", prefix = "ar", default)]
	pub comprador: Vec<Comprador>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Comprador",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct Comprador {
	#[yaserde(rename = "DocTipo", prefix = "ar", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "ar", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "Porcentaje", prefix = "ar", default)]
	pub porcentaje: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Periodo",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct Periodo {
	#[yaserde(rename = "FchDesde", prefix = "ar", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "ar", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfActividad",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct ArrayOfActividad {
	#[yaserde(rename = "Actividad", prefix = "ar", default)]
	pub actividad: Vec<Actividad>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Actividad",
	//namespace = "tns: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar",
)]
struct Actividad {
	#[yaserde(rename = "Id", prefix = "ar", default)]
	pub id: i64, 
}

//**************************************************************************** */
//***************************Respuesta**************************************** */
//**************************************************************************** */

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitarResponse",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "ar: ",
)]
struct FecaesolicitarResponse {
	#[yaserde(rename = "FECAESolicitarResult", prefix = "ar", default)]
	pub fecae_solicitar_result: Option<Fecaeresponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEResponse",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct Fecaeresponse {
	#[yaserde(rename = "FeCabResp", prefix = "ar", default)]
	pub fe_cab_resp: Option<FecaecabResponse>, 
	#[yaserde(rename = "FeDetResp", prefix = "ar", default)]
	pub fe_det_resp: Option<ArrayOfFECAEDetResponse>, 
	#[yaserde(rename = "Events", prefix = "ar", default)]
	pub events: Option<ArrayOfEvt>, 
	#[yaserde(rename = "Errors", prefix = "ar", default)]
	pub errors: Option<ArrayOfErr>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAECabResponse",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct FecaecabResponse {
	#[yaserde(flatten, default)]
	pub fecab_response: FecabResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECabResponse",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct FecabResponse {
	#[yaserde(rename = "Cuit", prefix = "ar", default)]
	pub cuit: i64, 
	#[yaserde(rename = "PtoVta", prefix = "ar", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "ar", default)]
	pub cbte_tipo: i32, 
	#[yaserde(rename = "FchProceso", prefix = "ar", default)]
	pub fch_proceso: Option<String>, 
	#[yaserde(rename = "CantReg", prefix = "ar", default)]
	pub cant_reg: i32, 
	#[yaserde(rename = "Resultado", prefix = "ar", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Reproceso", prefix = "ar", default)]
	pub reproceso: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEDetResponse",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct ArrayOfFECAEDetResponse {
	#[yaserde(rename = "FECAEDetResponse", prefix = "ar", default)]
	pub fecae_det_response: Vec<FecaedetResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEDetResponse",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct FecaedetResponse {
	#[yaserde(rename = "Concepto", prefix = "ar", default)]
	pub concepto: i32, 
	#[yaserde(rename = "DocTipo", prefix = "ar", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "ar", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "CbteDesde", prefix = "ar", default)]
	pub cbte_desde: i64, 
	#[yaserde(rename = "CbteHasta", prefix = "ar", default)]
	pub cbte_hasta: i64, 
	#[yaserde(rename = "CbteFch", prefix = "ar", default)]
	pub cbte_fch: Option<String>, 
	#[yaserde(rename = "Resultado", prefix = "ar", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Observaciones", prefix = "ar", default)]
	pub observaciones: Option<ArrayOfObs>,
	#[yaserde(rename = "CAE", prefix = "ar", default)]
	pub cae: Option<String>, 
	#[yaserde(rename = "CAEFchVto", prefix = "ar", default)]
	pub cae_fch_vto: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfObs",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct ArrayOfObs {
	#[yaserde(rename = "Obs", prefix = "ar", default)]
	pub obs: Vec<Obs>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Obs",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct Obs {
	#[yaserde(rename = "Code", prefix = "ar", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "ar", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEvt",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct ArrayOfEvt {
	#[yaserde(rename = "Evt", prefix = "ar", default)]
	pub evt: Vec<Evt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Evt",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct Evt {
	#[yaserde(rename = "Code", prefix = "ar", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "ar", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfErr",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct ArrayOfErr {
	#[yaserde(rename = "Err", prefix = "ar", default)]
	pub err: Vec<Err>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Err",
	namespace = "ar: http://ar.gov.afip.dif.FEV1/",
	prefix = "ar: ",
)]
struct Err {
	#[yaserde(rename = "Code", prefix = "ar", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "ar", default)]
	pub msg: Option<String>, 
}
