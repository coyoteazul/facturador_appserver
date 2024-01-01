use rocket_db_pools::sqlx::{Error, Acquire, query};
use rocket_db_pools::Connection;
use crate::Db;
use crate::types::Factura;

use super::factura_head::db_factura_head_alta;
use super::factura_producto::db_factura_producto_alta;


pub async fn db_factura_alta(
	db: &mut Connection<Db>, 
	factura:&mut Factura
) -> Result<bool, Error> {
	dbg!("Function call");
	let mut trans = db.begin().await?;
	
	db_factura_head_alta(&mut trans, factura).await?;

	for i in &factura.productos {
		db_factura_producto_alta(&mut trans, factura.id_factura, i).await?;
	}

	trans.commit().await?;

	return Ok(true);
}

pub async fn db_factura_set_cae (
	db: &mut Connection<Db>, 
	factura:&mut Factura
) -> Result<bool, Error> {
	dbg!("Function call");
	
	const QRY:&str = 
		"update factura set 
		cae = $2, 
		venc_cae = $3
		where id_factura = $1";

	let qry = query(QRY)
			/* 1*/.bind(&factura.id_factura)
			/* 2*/.bind(factura.cae)
			/* 3*/.bind(factura.venc_cae)
		;
		
	qry.execute(&mut ***db).await?;
	return Ok(true);
}
