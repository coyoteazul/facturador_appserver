
use rocket_db_pools::{sqlx::{query, Error, Row}, Connection};
use crate::Db;

pub enum OPERACION {
	ENVIO = 1,
	RESPUESTA = 2
}

pub async fn db_factura_transmision(
	db: &mut Connection<Db>, 
	id_factura: i32,
	id_transmision: Option<i32>,
	tipo: OPERACION,
	xml: &str
) -> Result<i32, Error> {
	dbg!("Function call");
	const QRY_ENVIO:&str = 
		"insert into factura_transmision 
			(id_factura, xml, id_operacion) Values	
			($1        , $2 , $3          )
			returning trans_ver";
	const QRY_RESPU:&str = 
		"insert into factura_transmision 
			(id_factura, xml, id_operacion, trans_ver) Values	
			($1        , $2 , $3          , $4       )
			returning trans_ver";
	let tipo: i16 = tipo as i16;
	let es_envio = tipo == OPERACION::ENVIO as i16;
	
	let qry_str = if es_envio {QRY_ENVIO} else {QRY_RESPU};
	let mut qry = query(qry_str)
		/* 1*/.bind(id_factura)
		/* 2*/.bind(xml)
		/* 3*/.bind(tipo);
	if !es_envio {
		/* 4*/qry = qry.bind(id_transmision.unwrap());
	};
		
	let row = qry.fetch_one(&mut ***db).await?;
	return Ok(row.get("trans_ver"));
}
