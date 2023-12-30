use std::{path::PathBuf, fs::File};

use crate::{model::afip::{cert_filepath, key_filepath}, conf::conf_filepath};


/** Verifica que existan los certificados de afip y el conf.json */
pub fn init_check() -> bool {
	let conf_json = try_open(&conf_filepath());
	let certificado_pem = try_open(&cert_filepath());
	let mi_clave_privada_key = try_open(&key_filepath());
		
	return conf_json && certificado_pem && mi_clave_privada_key;
}

fn try_open(file_path:&PathBuf) -> bool {
	let mut retorno = true;
	let file = File::open(file_path);
	file.err().map(|err|{
		println!("--No se pudo abrir el archivo: {} :||: {:?}",
			file_path.display().to_string(), err);
			println!("");
		retorno = false;
	});

	return retorno;
}