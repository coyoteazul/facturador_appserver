use chrono::format::parse;
use reqwest::{Client, header::CONTENT_TYPE};
use yaserde_derive::{YaSerialize,YaDeserialize};
use yaserde;

pub async fn afip_dummy(req_cli: &Client) -> Result<FEDummyResult, reqwest::Error>{
	

	let url = "https://wswhomo.afip.gov.ar/wsfev1/service.asmx?op=FEDummy";
	let request = req_cli.post(url)
	.header(CONTENT_TYPE, "application/soap+xml")
	.body(
		"<?xml version=\"1.0\" encoding=\"utf-8\"?>
		<soap12:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" xmlns:soap12=\"http://www.w3.org/2003/05/soap-envelope\">
			<soap12:Body>
				<FEDummy xmlns=\"http://ar.gov.afip.dif.FEV1/\" />
			</soap12:Body>
		</soap12:Envelope>")
	.send().await?;

	let response = request.text().await?;
	let cropped = crop_xml_declaration(&response);
	let parsed:Result<FedummyResponse, ()> = yaserde::de::from_str(&cropped).map_err(|e| {
		println!("Deserialization error: {:?}", e);
	});
	let after_parse:FedummyResponse;

	match parsed {
		Ok(a) => {
			after_parse = a;
		}
		Err(e) => {
			println!("parsed err:{:?}",e);
			after_parse = FedummyResponse{ fe_dummy_result: todo!() };
		}

	}

	
	println!("parsed:{:?}",after_parse);

	return Ok(response);


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


