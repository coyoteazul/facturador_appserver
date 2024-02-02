use std::{path::PathBuf, env};

use config::Config;
use rocket::serde::Deserialize;


pub fn conf_filepath() ->PathBuf {
	const FILE:&str = "./Conf.json";
	return env::current_exe().unwrap().parent().unwrap().join(FILE);
}


impl Conf {
	pub fn new() -> Self {
			let conf_path = conf_filepath().display().to_string();
			let builder = Config::builder()
			.add_source(config::File::with_name(&conf_path))
			.build().unwrap();

			let setting:Conf = builder.try_deserialize().unwrap();

			let db_url = format!("{}{}{}",
			 r#"{my_postgres_db={url=""#,setting.database.url,r#""}}"#);
			std::env::set_var("ROCKET_DATABASES", db_url);
			dbg!("ROCKET_DATABASES: {}", std::env::var("ROCKET_DATABASES").unwrap());

			std::env::set_var("ROCKET_PORT", setting.server.port.to_string());

			return setting;
	}
	pub fn is_prd(&self) -> bool {
		return self.entorno.is_prd;
	}
}


#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Conf {
    pub server: Server,
    pub afip_com: AfipCom,
    pub entorno: Entorno,
		pub database:Database
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Server {
  pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Entorno {
	pub is_prd: bool,
	pub impresora:String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct AfipCom {
	pub cuit: i64,
	pub iibb_num: i64,
	pub nombre_fantasia: String,
	pub direccion: String,
	pub inicio_actividad: String,
	pub iva_responsable: String,

}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Database {
	url: String
}