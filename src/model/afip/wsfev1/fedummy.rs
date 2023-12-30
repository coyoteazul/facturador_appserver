use reqwest::Client;
use yaserde_derive::{YaSerialize,YaDeserialize};
use yaserde;

use crate::model::afip::soap_utils::afip_post;

pub async fn afip_dummy(
	req_cli: &Client
) -> Result<FEDummyResult, reqwest::Error>{
	let xml_body = afip_post(req_cli, false, "FEDummy","", false, None).await?;

	let parsed:Result<FedummyResponse, ()> = yaserde::de::from_str(&xml_body).map_err(|e| {
		println!("Deserialization error: {:?}", e);
	});
	let after_parse:FedummyResponse;

	match parsed {
		Ok(a) => {
			after_parse = a;
			println!("parsed:{:?}",after_parse);
		}
		Err(e) => {
			println!("parsed err:{:?}",e);
			after_parse = FedummyResponse{ fe_dummy_result: todo!() };
		}

	}

	return Ok(after_parse.fe_dummy_result);
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


