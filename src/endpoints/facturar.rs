use reqwest::Client;
use rocket::{serde::{json::{Json, Value, self}}, http::Status, State};
use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::error::ErrorKind::UniqueViolation;
use crate::{types::Factura, model::propio::factura::db_factura_alta, Db};

#[post("/facturar", data = "<input>")]
pub async fn facturar(
	db: Connection<Db>, input: Json<Factura>, req_cli: &State<Client>
)  -> Status { 
	println!("facturar input{:?}", input);

	//Alta de head e items en la base
	let res = db_factura_alta(db, &input.0).await;

	match res {
		Ok(msg) => {
			println!("facturar ok:{:?}", msg);
			return Status::Ok;
		}
		Err(e) => {
			let a = e.as_database_error().unwrap();
			println!("facturar err:{:?}", a);
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