use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Cliente {
	pub tipo_doc:i8,
	pub num_doc:i32,
	pub nombre:String
}
