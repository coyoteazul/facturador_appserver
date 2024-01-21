use rocket::{serde::{Deserialize, json::{Value, self}}, http::Status};
use rocket_db_pools::Connection;

use crate::{model::propio::cliente::db_cliente_get, Db};

#[get("/cliente/<tipo_doc>/<num_doc>")]
pub async fn cliente_get(
	mut db: Connection<Db>,
	tipo_doc:i32,
	num_doc:i64
) -> Result<Value, Status> { 
	//dbg!("cliente_get input:", &input);
	let res = db_cliente_get(&mut db, tipo_doc, num_doc).await;
	dbg!(&res);

	match res {
		Ok(cli) => {
			let reto = json::to_value(&cli).unwrap();
			return Ok(reto);
		}
		Err(_) => {
			return Err(Status::NoContent);
		}
	}
	//return input;
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ClienteGet {
	pub tipo_doc:i32,
	pub num_doc:i64
}
