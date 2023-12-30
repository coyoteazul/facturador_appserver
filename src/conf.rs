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
			 "{my_postgres_db={url=\"",setting.database.url,"\"}}");

			std::env::set_var("ROCKET_DATABASES", db_url);
			println!("ROCKET_DATABASES: {}", std::env::var("ROCKET_DATABASES").unwrap());
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
	pub is_prd: bool
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct AfipCom {
	pub cuit: String,
	pub nombre: String,
	pub direccion: String
}

#[derive(Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Database {
	url: String
}