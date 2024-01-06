use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct Producto {
	pub id_producto:i32,
	pub nombre:String,
	pub unidad_medida:String,
	pub producto_grupo:String,
	#[serde(default)]
	pub precio:f64
}