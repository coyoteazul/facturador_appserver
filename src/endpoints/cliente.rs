use rocket::{serde::{Deserialize, json::{Json, Value, self}}, http::Status};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::error::ErrorKind::UniqueViolation;
use crate::{model::propio::cliente::{db_cliente_get, db_cliente_alta}, Db, types::Cliente};

#[get("/cliente", data = "<input>")]
pub async fn cliente_get(
	db: Connection<Db>, input: Json<ClienteGet>
) -> Result<Value, Status> { 
	println!("cliente_get input:{:?}", input);
	let res = db_cliente_get(db, input.tipo_doc, input.num_doc).await;
	println!("{:?}", res);

	match res {
		Ok(cli) => {
			let reto = json::to_value(&cli).unwrap();
			return Ok(reto);
		}
		Err(e) => {
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

#[post("/cliente", data = "<input>")]
pub async fn cliente_alta(
	db: Connection<Db>, input: Json<Cliente>
) -> Status { 
	println!("cliente_alta input:{:?}", input);
	let res = db_cliente_alta(db, input.0).await;
	println!("{:?}", res);

	match res {
		Ok(msg) => {
			println!("cliente_alta ok:{:?}", msg);
			return Status::Ok;
		}
		Err(e) => {
			let a = e.as_database_error().unwrap();
			println!("cliente_alta err:{:?}", a);
			match a.kind() {
				UniqueViolation =>{
					return Status::Conflict;
				}
				_ => {
					return Status::InternalServerError;
				}
			}
		}
	}
	//return input;
}