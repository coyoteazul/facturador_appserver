use rocket::{serde::json::{Value, self}, http::Status};
use rocket_db_pools::Connection;
use crate::{model::propio::producto_grupo::db_producto_grupo_get_all, Db};

#[get("/producto_grupo")]
pub async fn producto_grupo_get(
	mut db: Connection<Db>
) -> Result<Value, Status> { 
	dbg!("producto_grupo_get input");
	let res = db_producto_grupo_get_all(&mut db).await;
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
