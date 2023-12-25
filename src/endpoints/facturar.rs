use rocket::serde::json::Json;
use crate::types::Factura;

#[post("/facturar", data = "<input>")]
pub fn facturar(input: Json<Factura>) { 
	println!("{:?}", input);

	//return input;
}