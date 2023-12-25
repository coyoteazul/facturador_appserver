use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct FacturaProduct {
	pub item_num:i32,
	pub id_producto:i32,
	pub cantidad:f32,
	pub precio:f32
}