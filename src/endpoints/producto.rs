use rocket::{serde::json::{Value, self}, http::Status};
use rocket_db_pools::Connection;
use crate::{model::propio::producto::db_producto_get_all, Db};

#[get("/producto")]
pub async fn producto_get(
	mut db: Connection<Db>
) -> Result<Value, Status> { 
	dbg!("producto_get input");
	let res = db_producto_get_all(&mut db).await;
	dbg!(&res);

	match res {
		Ok(prod) => {
			let reto = json::to_value(&prod).unwrap();
			return Ok(reto);
		}
		Err(_) => {
			return Err(Status::NoContent);
		}
	}
	//return input;
}
