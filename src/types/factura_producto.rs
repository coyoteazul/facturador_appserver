use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FacturaProduct {
	pub item_num:i32,
	pub id_producto:i32,
	pub cantidad:f32,
	pub valor:f64
}