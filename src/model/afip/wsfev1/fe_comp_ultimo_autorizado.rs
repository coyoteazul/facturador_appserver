use crate::{model::afip::soap_utils::{afip_post, get_xml_tag, afip_signin}, CONF};
use rocket_db_pools::Connection;
use crate::Db;
use reqwest::{Client};
use yaserde_derive::{YaSerialize,YaDeserialize};

use super::constants::{WSFEV1_PRD, WSFEV1_VAL};

pub async fn fe_comp_ultimo_autorizado (
	req_cli: &Client, db: &mut Connection<Db>, tipo_fac: i32, p_vta: i32
) -> Result<i64, String> {
	let url = if CONF.is_prd() {WSFEV1_PRD} else {WSFEV1_VAL};
	
	match afip_signin(req_cli, "wsfe",db).await {
		Ok(auth) => {
			let soap_msg = make_soap_msg(auth, tipo_fac, p_vta);
			let respuesta = afip_post(req_cli, url,soap_msg).await;

			match respuesta {
				Ok(res) => {
					let body = get_xml_tag(&res, "soap:Body")
						.replace("xmlns=\"http://ar.gov.afip.dif.FEV1/\"", "");

					let parsed:Result<FecompUltimoAutorizadoResponse, String> = 
						yaserde::de::from_str(&body).map_err(|e| {
							return format!("Deserialization error: {:?}", e);
						});
					return Ok(parsed.unwrap().fe_comp_ultimo_autorizado_result.unwrap().cbte_nro);
				}
				Err(e) => {
					dbg!(&e);
					return Err(e.to_string());
				}
			}
		}
		Err(e) => {
			dbg!(&e);
			return Err(e);
		}
	}
}

fn make_soap_msg(
	auth:String, tipo_fac: i32, p_vta: i32
) -> String {
	return format!(r#"
	<soap:Envelope 
		xmlns:soap="http://www.w3.org/2003/05/soap-envelope" 
		xmlns:ar="http://ar.gov.afip.dif.FEV1/">
		<soap:Header/>
		<soap:Body>
		<ar:FECompUltimoAutorizado>
			{auth}
			<ar:PtoVta>{p_vta}</ar:PtoVta>
			<ar:CbteTipo>{tipo_fac}</ar:CbteTipo>
		</ar:FECompUltimoAutorizado>	
		</soap:Body>
		</soap:Envelope>"#
	);
}




#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompUltimoAutorizadoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
struct FecompUltimoAutorizadoResponse {
	#[yaserde(rename = "FECompUltimoAutorizadoResult", prefix = "wsfev1", default)]
	pub fe_comp_ultimo_autorizado_result: Option<FerecuperaLastCbteResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FERecuperaLastCbteResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
struct FerecuperaLastCbteResponse {
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
	#[yaserde(rename = "CbteNro", prefix = "wsfev1", default)]
	pub cbte_nro: i64, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
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
