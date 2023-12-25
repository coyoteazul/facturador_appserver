use reqwest::{Client, header::CONTENT_TYPE};
use yaserde;
use yaserde_derive::{YaSerialize,YaDeserialize};

pub async fn afip_dummy(req_cli: &Client) -> Result<String, reqwest::Error>{
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
	let parsed:FeDummyResult = yaserde::de::from_str(&cropped).unwrap(); //.map_err(Error::Deserialization);

	println!("response:{}",response);
	println!("cropped:{}",cropped);
	println!("parsed:{:?}",parsed);

	return Ok(response);


}

fn crop_xml_declaration(xml: &str) -> String {
	xml.split("?>").skip(1).collect()
}


#[derive(Debug, YaSerialize, YaDeserialize,Default)]
#[yaserde(prefix = "soap", namespace = "http://www.w3.org/2003/05/soap-envelope")]
struct SoapBody {
    #[yaserde(prefix = "FEDummyResponse", namespace = "http://www.w3.org/2003/05/soap-envelope")]
    fe_dummy_response: FeDummyResponse,
}

#[derive(Debug, YaSerialize, YaDeserialize,Default)]
#[yaserde(prefix = "FEDummyResponse", namespace = "http://www.w3.org/2003/05/soap-envelope")]
struct FeDummyResponse {
    #[yaserde(prefix = "FEDummyResult", namespace = "http://www.w3.org/2003/05/soap-envelope")]
    fe_dummy_result: FeDummyResult,
}

#[derive(Debug, YaSerialize, YaDeserialize,Default)]
#[yaserde(prefix = "FEDummyResult", namespace = "http://www.w3.org/2003/05/soap-envelope")]
struct FeDummyResult {
    app_server: String,
    db_server: String,
    auth_server: String,
}