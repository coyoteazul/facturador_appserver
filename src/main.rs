mod endpoints;
mod types; 
mod aux_func;
mod model;
mod conf;
mod init_check;



use std::{path::PathBuf, env};

use init_check::init_check;
use lazy_static::lazy_static;

use rocket_db_pools::Database;
use sqlx;
use rocket::{launch, routes, fs::FileServer};
use conf::Conf;
use local_ip_address::local_ip;



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
	println!("Direccion del frontend:{:?}:8000",local_ip().unwrap());


	let cli = reqwest::Client::builder()
		.user_agent("a")
		.build()
		.unwrap();

	rocket::build()
		.manage(cli)
		.attach(Db::init())
		.attach(make_cors()) // 7.
		.mount("/", FileServer::from(frontend_filepath())) //copiar dist del proyecto react, y renombrarlo a front_end
		.mount("/endback", routes![
			endpoints::facturar::facturar,
			endpoints::cliente::cliente_get,
			endpoints::producto::producto_get,
			endpoints::producto_grupo::producto_grupo_get])
}


use rocket::http::Method; // 1.

use rocket_cors::{
    AllowedHeaders, AllowedOrigins, // 2.
    Cors, CorsOptions // 3.
};

fn make_cors() -> Cors {
    /*let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
        "http://localhost:8081",
        "http://127.0.0.1:8081",
        "http://localhost:8000",
        "http://0.0.0.0:8000",
    ]);*/
		let allowed_origins = AllowedOrigins::All;

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
						"content-type",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

pub fn frontend_filepath() ->PathBuf {
	dbg!("Function call");
	const FILE:&str = "./front_end";
	return env::current_exe().unwrap().parent().unwrap().join(FILE);
}