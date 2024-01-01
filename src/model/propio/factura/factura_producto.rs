use rocket_db_pools::sqlx::{query, Error, Transaction, Postgres};
use rocket_db_pools::sqlx::postgres::PgQueryResult;
use crate::types::FacturaProduct;

pub async fn db_factura_producto_alta(
	trans: &mut Transaction<'_, Postgres>, id_factura:i32, producto:&FacturaProduct	 
) -> Result<PgQueryResult, Error> {
	dbg!("Function call");
	const QRY:&str = 
		"insert into factura_producto (id_factura, item_num, id_producto, cantidad, valor)
		 values                       ($1        , $2      , $3         , $4      , $5    )";

	let qry = query(QRY)
			/* 1*/.bind(id_factura)
			/* 2*/.bind(producto.item_num)
			/* 3*/.bind(producto.id_producto)
			/* 4*/.bind(producto.cantidad)
			/* 5*/.bind(producto.valor)
		;
		
	let retorno = qry.execute(&mut **trans).await;
	return retorno;
}