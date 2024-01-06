use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct ProductoGrupo {
	pub id_producto_grupo:i32,
	pub nombre:String,
}