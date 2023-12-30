use reqwest::{Client, header::CONTENT_TYPE};

pub async fn afip_post(
	req_cli: &Client, url:&str, soap_msg:String
) -> Result<String, reqwest::Error> {

	let request = req_cli.post(url)
	.header(CONTENT_TYPE, "application/soap+xml")
	.body(soap_msg)
	.send().await?;

	let response = request.text().await?;
	println!("afip_post response:{}",response);
	return Ok(response);
}




