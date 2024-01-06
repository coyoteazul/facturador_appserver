use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MedioPago {
	pub id_medio_pago:i32,
	pub nombre:String,
	pub es_electronico:bool
}