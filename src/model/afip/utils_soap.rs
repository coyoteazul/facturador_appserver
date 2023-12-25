use reqwest::{Client, header::CONTENT_TYPE};
use yaserde::{YaDeserialize, YaSerialize};



pub async fn afip_post(
	req_cli: &Client, is_prd:bool, endpoint:&str, req_auth:bool, extra_body:Option<&str>
) -> Result<String, reqwest::Error> {
	let url;
	if is_prd {
		url = format!("https://wswhomo.afip.gov.ar/wsfev1/service.asmx?op={}",endpoint);
	} else {
		url = format!("https://wswhomo.afip.gov.ar/wsfev1/service.asmx?op={}",endpoint);
	}

	let body = body_maker(endpoint, if(req_auth) {Some(auth_maker())} else {None}, extra_body);
	println!("url:{}",url);
	println!("body:{}",body);
	
	let request = req_cli.post(url)
	.header(CONTENT_TYPE, "application/soap+xml")
	.body(body)
	.send().await?;

	let response = crop_xml_declaration(request.text().await?);
	println!("response:{}",response);
	return Ok(response);
}

fn body_maker(
	endpoint:&str, auth:Option<AfipAuth>, extra:Option<&str>
) -> String {
	let auth_str;
	let extra_str = extra.unwrap_or("");
	match auth {
		Some(o) => {
			auth_str = format!(
			"<Auth>
				<Token>{}</Token>
				<Sign>{}</Sign>
				<Cuit>{}</Cuit>
			</Auth>", o.token, o.sign, o.cuit.to_string());		
		}
		None => {
			auth_str = String::new();
		}
	}

	return format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>
	<soap12:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\" xmlns:soap12=\"http://www.w3.org/2003/05/soap-envelope\">
		<soap12:Body>
			<{} xmlns=\"http://ar.gov.afip.dif.FEV1/\" />
			{}
		</soap12:Body>
	</soap12:Envelope>",
	endpoint, auth_str);
}

struct AfipAuth {
	token:String,
	sign:String,
	cuit:i32
}

fn auth_maker() -> AfipAuth {
	//TODO no hardcodear esto
	return AfipAuth{
		token:String::from("aaa"),
		sign:String::from("aab"),
		cuit:0
	}
}

fn crop_xml_declaration(xml: String) -> String {
	let start_tag = "<soap:Body>";
	let end_tag = "</soap:Body>";

	if let Some(start_index) = xml.find(start_tag) {
			if let Some(end_index) = xml.find(end_tag) {
					let body_content = &xml[start_index + start_tag.len()..end_index];
					return body_content.to_string();
			}
	}

	String::from("")
}