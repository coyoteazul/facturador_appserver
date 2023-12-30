use std::{sync::Arc, collections::HashMap, ops::Add};
use rocket::tokio::sync::Mutex;

use chrono::{Duration, DateTime, Local};
use reqwest::{header::CONTENT_TYPE, Client};
use yaserde_derive::{YaSerialize,YaDeserialize};
use lazy_static::lazy_static;
use crate::model::afip::soap_utils::{arma_login_ticket_request::arma_login_ticket_request_xml, cms_sign::sign_with_cms, get_soap_tag::get_soap_tag};

lazy_static! {
	static ref CACHE: Mutex<HashMap<String, AfipAuth>> = Mutex::new(HashMap::new());
	//static ref CACHE: Arc<Mutex<HashMap<String, AfipAuth>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn afip_signin(
	req_cli: &Client, service:&str, is_prd:bool
) -> String {
	println!("afip_signin para {service}");
	//check cache
	
	//let cach:&'static Mutex<HashMap<String, AfipAuth>> = unsafe { &CACHE };

	//let cloned = Arc::clone(&CACHE);
	let mut locked = CACHE.lock().await;

	if let Some(val) = locked.get(service) {
		if val.expir > chrono::Local::now().add(Duration::minutes(10)) {
			let retorno = auth_format(val);
			println!("Signin desde cache:{retorno}");
			return retorno;
		}
	}

	let req_date = Local::now() - Duration::minutes(5);
	let exp_date = req_date + Duration::hours(23);
	let req = arma_login_ticket_request_xml(service, req_date, exp_date);
	println!("{}", &req);
	let signed_req = sign_with_cms(&req);
	let url =	format!("https://{}.afip.gov.ar/ws/services/LoginCms?wsdl",
		if is_prd {"wsaa"} else {"wsaahomo"}
	);
	

	let body = format!(
	"<soapenv:Envelope xmlns:soapenv=\"http://schemas.xmlsoap.org/soap/envelope/\" xmlns:wsaa=\"http://wsaa.view.sua.dvadac.desein.afip.gov\">
		<soapenv:Header/>
		<soapenv:Body>
			<wsaa:loginCms>
					<wsaa:in0>{signed_req}</wsaa:in0>
			</wsaa:loginCms>
		</soapenv:Body>
	</soapenv:Envelope>");
	println!("body: {body}");

	let request = req_cli.post(url)
		.header(CONTENT_TYPE, "text/xml")
		.header("SOAPAction", "")
		.body(body)
		.send().await.unwrap();

	let response = request.text().await.unwrap();
	println!("response:{}",response);
	
	let obj = AfipAuth{ 
		token: get_soap_tag(&response, "token"),
		sign:  get_soap_tag(&response, "sign"), 
		expir: exp_date
	};
	let retorno = auth_format(&obj);
	println!("Signin desde consulta:{retorno}");
	locked.insert(String::from(service), obj);

	return retorno

}

struct AfipAuth {
	token:String,
	sign:String,
	expir:DateTime<Local>
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
struct LoginCmsResponse {
    #[yaserde(prefix = "tns", rename = "loginCmsResponse")]
    pub login_cms_return: String,
}

fn auth_format(o:&AfipAuth) ->String {
	return format!(
	"<Auth>
		<Token>{}</Token>
		<Sign>{}</Sign>
		<Cuit>{}</Cuit>
	</Auth>", o.token, o.sign, "20366025589");
}
