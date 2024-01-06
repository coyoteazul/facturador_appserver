use rocket::serde::{Deserialize, Serialize};

use super::Producto;

#[derive(Deserialize, Debug,Serialize)]
#[serde(crate = "rocket::serde")]
pub struct FacturaProduct {
	pub item_num:i32,
	pub producto:Producto,
	pub cantidad:f32,
	pub valor:f64
}