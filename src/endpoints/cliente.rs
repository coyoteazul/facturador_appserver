use rocket::{serde::{Deserialize, json::{Json, Value, self}}, http::Status};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::error::ErrorKind::UniqueViolation;
use crate::{model::propio::cliente::{db_cliente_get, db_cliente_alta}, Db, types::Cliente};

#[get("/cliente", data = "<input>")]
pub async fn cliente_get(
	mut db: Connection<Db>, input: Json<ClienteGet>
) -> Result<Value, Status> { 
	dbg!("cliente_get input:", &input);
	let res = db_cliente_get(&mut db, input.tipo_doc, input.num_doc).await;
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
