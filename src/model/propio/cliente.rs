use rocket_db_pools::sqlx::Error;
use rocket_db_pools::sqlx::postgres::PgQueryResult;
use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::Row;
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
	db: &mut Connection<Db>, cliente:Cliente 
) -> Result<PgQueryResult, Error> {
	let qry = 
		sqlx::query("insert into cliente (tipo_doc_cod, num_doc, nombre) 
			Values($1,$2,$3)")
		.bind(cliente.tipo_doc)
		.bind(cliente.num_doc)
		.bind(cliente.nombre);
	
	let retorno = qry.execute(&mut ***db).await;
	dbg!("db_cliente_alta:{:?}", &retorno);
	return retorno
}