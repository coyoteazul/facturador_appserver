use rocket_db_pools::sqlx::Error;

use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::Row;
use sqlx::{Transaction, Postgres};
use crate::Db;
use crate::types::Cliente;

pub async fn db_cliente_get(
	db: &mut Connection<Db>, tipo_doc: i32, num_doc: i64
) -> Result<Cliente, Error> {
	let qry = 
		sqlx::query("SELECT nombre FROM cliente 
		WHERE tipo_doc_cod = $1 and num_doc = $2")
		.bind(tipo_doc)
		.bind(num_doc);
	
	return qry.fetch_one(&mut ***db).await
		.and_then(|r| Ok(Cliente{
				nombre: r.try_get(0)?, tipo_doc, num_doc
		}));
}

pub async fn db_cliente_alta(
	trans: &mut Transaction<'_, Postgres>, 
	cliente:&Cliente
) -> Result<bool, Error> {
	dbg!("Function call");

	if cliente.tipo_doc == 0 {
		return Ok(true);
	}

	const QRY:&str = 
		"insert into cliente 
			(tipo_doc_cod, num_doc, nombre) Values
			($1          , $2     , $3    )
			on conflict (tipo_doc_cod, num_doc) do nothing ";

	let qry = sqlx::query(QRY)
		/* 1*/.bind(cliente.tipo_doc)
		/* 2*/.bind(cliente.num_doc)
		/* 3*/.bind(&cliente.nombre);
		
	let _ = qry.execute(&mut **trans).await?;
	return Ok(true);
}