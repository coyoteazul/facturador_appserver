
use rocket_db_pools::{sqlx::{query, Error}, Connection};
use crate::{Db};
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%:z";

pub enum OPERACION {
	ENVIO = 1,
	RESPUESTA = 2
}

pub async fn db_factura_transmision(
	db: &mut Connection<Db>, 
	id_factura: i32,
	id_transmision: i32,
	tipo: OPERACION,
	xml: &str
) -> Result<bool, Error> {
	const QRY:&str = 
		"insert into factura_transmision 
			(id_factura, id_operacion, archivo, id_transmision) Values	
			($1        , $2          , $3::XML, $4)";
	let qry = query(QRY)
			/* 1*/.bind(id_factura)
			/* 2*/.bind(tipo as i32)
			/* 3*/.bind(xml)
			/* 4*/.bind(id_transmision)	
		;
		
	qry.execute(&mut ***db).await?;
	return Ok(true);
}
