use std::{sync::Arc, collections::HashMap, ops::Add, env};
use rocket::tokio::sync::{Mutex, RwLock};

use chrono::{Duration, DateTime, Local};
use reqwest::{header::CONTENT_TYPE, Client};
use yaserde_derive::{YaSerialize,YaDeserialize};
use lazy_static::lazy_static;
use crate::{model::afip::soap_utils::{arma_login_ticket_request::arma_login_ticket_request_xml, cms_sign::sign_with_cms, get_xml_tag::get_xml_tag}, CONF};

lazy_static! {
	//static ref CACHE: Mutex<HashMap<String, AfipAuth>> = Mutex::new(HashMap::new());
	static ref CACHE: RwLock<HashMap<String, AfipAuth>> = RwLock::new(HashMap::new());
}

pub async fn afip_signin(
	req_cli: &Client, service:&str
) -> String {
	println!("afip_signin para {service}");
	
	//check cache
	if let Some(val) = CACHE.read().await.get(service) {
		if val.expir > chrono::Local::now().add(Duration::minutes(10)) {
			let retorno = val.xml.clone();
			println!("Signin desde cache:{retorno}");
			return retorno;
		}
	}

	let mut locked = CACHE.write().await;

	let req_date = Local::now() - Duration::minutes(5);
	let exp_date = req_date + Duration::hours(23);
	let req = arma_login_ticket_request_xml(service, req_date, exp_date);
	println!("{}", &req);
	let signed_req = sign_with_cms(&req);
	let url =	format!("https://{}.afip.gov.ar/ws/services/LoginCms?wsdl",
		if CONF.is_prd() {"wsaa"} else {"wsaahomo"}
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
	
	let obj = AfipAuth::new( 
		get_xml_tag(&response, "token"),
		get_xml_tag(&response, "sign"), 
		exp_date
	);
	let retorno = obj.xml.clone();
	println!("Signin desde consulta:{retorno}");
	locked.insert(String::from(service), obj);

	return retorno
}

struct AfipAuth {
	xml:String,
	expir:DateTime<Local>
}

impl AfipAuth {
	pub fn new(	
		token:String, sign:String, expir:DateTime<Local>
	) ->Self {
		Self{
			xml: format!(
				"<Auth>
					<Token>{}</Token>
					<Sign>{}</Sign>
					<Cuit>{}</Cuit>
				</Auth>", token, sign, CONF.afip_com.cuit),
			expir
		}
	}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
struct LoginCmsResponse {
    #[yaserde(prefix = "tns", rename = "loginCmsResponse")]
    pub login_cms_return: String,
}

