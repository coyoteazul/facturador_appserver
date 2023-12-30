mod endpoints;
mod types; 
mod aux_func;
mod model;
mod conf;
mod init_check;
use core::panic;
use std::env;

use init_check::init_check;
use lazy_static::lazy_static;

use rocket_db_pools::{sqlx, Database};
use rocket::{launch, routes};
use conf::Conf;

use crate::model::afip::wsfev1::afip_dummy;

lazy_static! {
	//static ref Conf: RwLock<Conf> = RwLock::new(Conf::new());
	static ref CONF: Conf = Conf::new();	
}
 
#[derive(Database)]
#[database("my_postgres_db")]
pub struct Db(sqlx::PgPool);

#[macro_use] extern crate rocket;
#[launch]
async fn rocket() -> _ {
	if !init_check() {
		let _ = std::process::exit(0);
	}

	println!("Iniciando entorno: {}", if CONF.is_prd() {"PRD"} else {"VAL"} );

	let cli = reqwest::Client::builder()
		.user_agent("a")
		.build()
		.unwrap();

	rocket::build()
		.manage(cli)
		.attach(Db::init())
		.mount("/", routes![endpoints::facturar::facturar])
		.mount("/", routes![endpoints::cliente::cliente_get])
		.mount("/", routes![endpoints::cliente::cliente_alta])
}
