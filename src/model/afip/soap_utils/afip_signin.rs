use std::{collections::HashMap, ops::Add};
use rocket::tokio::sync::RwLock;

use chrono::{Duration, DateTime,  Utc};
use reqwest::{header::CONTENT_TYPE, Client};
use rocket_db_pools::Connection;
use lazy_static::lazy_static;
use crate::{model::{afip::{soap_utils::{arma_login_ticket_request::arma_login_ticket_request_xml, cms_sign::sign_with_cms, get_xml_tag::get_xml_tag, soap_fault::SoapFault}, afip_sign_cache::{db_afip_sign_cache_get, db_afip_sign_cache_alta}}}, CONF, types::AfipAuth, Db};

lazy_static! {
	//static ref CACHE: Mutex<HashMap<String, AfipAuth>> = Mutex::new(HashMap::new());
	static ref CACHE: RwLock<HashMap<String, AfipAuth>> = RwLock::new(HashMap::new());
}

pub async fn afip_signin(
	req_cli: &Client, service:&str, db: &mut Connection<Db>
) -> Result<String, String> {
	dbg!("afip_signin para ",service);

	match login_desde_cache(db, service).await {
		Some(auth) => {
			return Ok(auth);
		}
		None => {
			login_online(db, service,req_cli).await?;
			return Ok(login_desde_cache(db, service).await.unwrap());
		}
	};
}

async fn login_desde_cache(
	db: &mut Connection<Db>,
	service:&str
) -> Option<String> {
	dbg!("Function call");
	let cache_len = 'bloque: {
		let cache_ref = &*CACHE.read().await;
		break 'bloque cache_ref.len();
	};
	
	/*Buscar cache en base de datos */ 
	if cache_len == 0 {
		let db_cache = db_afip_sign_cache_get(db).await;
		if db_cache.len() != 0 {
			let mut locked = CACHE.write().await;
			*locked = db_cache;
		}
	}
	let cache_ref = &*CACHE.read().await;
	if let Some(val) = cache_ref.get(service) {
		if val.expir > chrono::Utc::now().add(Duration::minutes(10)) {
			dbg!("Signin desde cache:",&val.xml);
			return Some(val.xml.clone());
		}
	}
	
	return None;
}

async fn login_online(
	db: &mut Connection<Db>,
	service:&str,
	req_cli: &Client
) -> Result<(), String> {
	dbg!("Function call");
	let req_date = Utc::now() - Duration::minutes(5);
	let exp_date = req_date + Duration::hours(23);
	let req = arma_login_ticket_request_xml(service, req_date, exp_date);
	dbg!(&req);
	let signed_req = sign_with_cms(&req);
	let url =	format!("https://{}.afip.gov.ar/ws/services/LoginCms?wsdl",
		if CONF.is_prd() {"wsaa"} else {"wsaahomo"}
	);
	
	let body = format!(r#"
	<soapenv:Envelope 
	xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" 
	xmlns:wsaa="http://wsaa.view.sua.dvadac.desein.afip.gov">
		<soapenv:Header/>
		<soapenv:Body>
			<wsaa:loginCms>
					<wsaa:in0>{signed_req}</wsaa:in0>
			</wsaa:loginCms>
		</soapenv:Body>
	</soapenv:Envelope>"#);
	dbg!(&body);

	let request = req_cli.post(url)
		.header(CONTENT_TYPE, "text/xml")
		.header("SOAPAction", "")
		.body(body)
		.send().await.unwrap();

	let response = request.text().await.unwrap();
	dbg!(&response);
	if response.contains("<faultcode") {
		return Err(format!("{:?}",SoapFault::new(&response)));
	};
	
	let expir_str = get_xml_tag(&response, "expirationTime");
	let expir = DateTime::parse_from_str(&expir_str,"%Y-%m-%dT%H:%M:%S%.f%:z").unwrap().with_timezone(&Utc);
	
	let obj = AfipAuth::new( 
		get_xml_tag(&response, "token"),
		get_xml_tag(&response, "sign"), 
		expir ,
	);

	if obj.xml.len()!= 0 {
		/*Lock de cache*/{
			let mut locked = CACHE.write().await;
			dbg!("Signin desde consulta:{}",&obj.xml);
			locked.insert(String::from(service), obj);
		}
		db_afip_sign_cache_alta(db, &*CACHE.read().await).await;
		return Ok(());
	}
	
	return Err(format!("Afip signin error: {}",response))
}

