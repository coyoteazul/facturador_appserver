use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::{Row, Postgres};
use crate::Db;
use crate::aux_func::time_serde::utc_from_ts_str;
use crate::types::AfipAuth;

pub async fn db_afip_sign_cache_get(
	db: &mut Connection<Db>
) -> HashMap<String, AfipAuth> {
	dbg!("Function call");
	const QRY:&str = 
		"select expir, servicio, xml_ 
		 from afip_sign_cache where expir > now()";
		
	let rows = sqlx::query(QRY).fetch_all(&mut ***db).await.unwrap();

	let mut retorno:HashMap<String, AfipAuth> = HashMap::new(); 
	for rec in rows {
		retorno.insert(
			rec.get("servicio"), 
			AfipAuth {
				xml: rec.get("xml_"),
				expir: rec.get("expir")
			});
			
	}

	return retorno;
}

pub async fn db_afip_sign_cache_alta(
	db: &mut Connection<Db>, src:&HashMap<String, AfipAuth>
) {
	dbg!("Function call");
	const QRY_TRUNC:&str = "truncate table afip_sign_cache";
	let _ = sqlx::query(QRY_TRUNC).execute(&mut ***db).await;

	const QRY_INS:&str = 
	"insert into afip_sign_cache 
		(expir          , servicio, xml_) values 
		($1::timestamptz, $2      , $3  )";

	for i in src {
		let qry = sqlx::query(QRY_INS)
		.bind(i.1.expir.to_string())
		.bind(i.0)
		.bind(&i.1.xml);

		qry.execute(&mut ***db).await;
	}
	
}
