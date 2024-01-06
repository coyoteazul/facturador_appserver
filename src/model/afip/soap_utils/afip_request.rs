use reqwest::{Client, header::CONTENT_TYPE};

pub async fn afip_post(
	req_cli: &Client, url:&str, soap_msg:String
) -> Result<String, reqwest::Error> {
	dbg!("Function call");
	
	dbg!("afip post url:", &url);
	dbg!("afip post soap_msg:", &soap_msg);
	
	let soap_msg = soap_msg.trim().to_string();
	let request = req_cli.post(url)
	.header(CONTENT_TYPE, "application/soap+xml")
	.body(soap_msg)
	.send().await?;

	let response = request.text().await?;
	dbg!("afip post response:", &response);
	return Ok(response);
}




