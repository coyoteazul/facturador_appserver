use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Cliente {
	pub tipo_doc:i32,
	pub num_doc:i64,
	pub nombre:String
}
