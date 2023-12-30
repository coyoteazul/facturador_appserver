use rocket_db_pools::sqlx::{Error, Acquire};
use rocket_db_pools::Connection;
use crate::Db;
use crate::types::Factura;

use super::factura_head::db_factura_head_alta;
use super::factura_producto::db_factura_producto_alta;


pub async fn db_factura_alta(
	mut db: Connection<Db>, factura:&Factura 
) -> Result<bool, Error> {
	let mut trans = db.begin().await?;
	let id_factura = db_factura_head_alta(&mut trans, factura).await?;

	for i in &factura.productos {
		db_factura_producto_alta(&mut trans, id_factura, i).await?;
	}

	trans.commit().await?;

	return Ok(true);
}