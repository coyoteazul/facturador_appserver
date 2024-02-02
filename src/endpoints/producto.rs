use rocket::{serde::json::{Value, self}, http::Status};
use rocket_db_pools::Connection;
use crate::{model::propio::producto::{db_producto_get_all, db_producto_get}, Db};

#[get("/producto?<id_producto>")]
pub async fn producto_get(
	mut db: Connection<Db>,
	id_producto:Option<i32>
) -> Result<Value, Status> { 
	dbg!("producto_get input");
	let res ;
	match id_producto {
		Some(id) => {
			res = db_producto_get(&mut db, id).await;
		} 
		None => {
			res = db_producto_get_all(&mut db).await;
		}
	}
	 
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
}
