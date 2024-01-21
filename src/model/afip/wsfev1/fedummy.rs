use reqwest::Client;
use yaserde_derive::{YaSerialize,YaDeserialize};
use yaserde;

use crate::{model::afip::{soap_utils::{afip_post, get_xml_tag}, wsfev1::constants::*}, CONF};

#[allow(dead_code)]
pub async fn afip_dummy(
	req_cli: &Client
) -> Result<FEDummyResult, reqwest::Error>{
	let url = if CONF.is_prd() {WSFEV1_PRD} else {WSFEV1_VAL};
  const XML_BODY:&str = r#"
	<soap:Envelope 
			xmlns:soap="http://www.w3.org/2003/05/soap-envelope" 
			xmlns:ar="http://ar.gov.afip.dif.FEV1/">
		<soap:Header/>
		<soap:Body>
			<ar:FEDummy/>
		</soap:Body>
	</soap:Envelope>"#;


	let respuesta = afip_post(req_cli, url,XML_BODY.to_string()).await?;

	let parsed:FedummyResponse = yaserde::de::from_str(&get_xml_tag(&respuesta, "soap:Body") )
	.map_err(|e| {
		dbg!("Deserialization error: {:?}", e);
	}).unwrap();
	
	return Ok(parsed.fe_dummy_result);
}



#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FedummyResponse {
    #[yaserde(prefix = "tns", rename = "FEDummyResult")]
    pub fe_dummy_result: FEDummyResult,
}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FEDummyResult {
    #[yaserde(prefix = "tns", rename = "AppServer")]
    pub app_server: String,

    #[yaserde(prefix = "tns", rename = "DbServer")]
    pub db_server: String,

    #[yaserde(prefix = "tns", rename = "AuthServer")]
    pub auth_server: String,
}


