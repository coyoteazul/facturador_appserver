use rocket_db_pools::sqlx::{query, Error, Transaction, Postgres, Row};
use crate::types::Factura;
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%:z";


pub async fn db_factura_head_alta(
	trans: &mut Transaction<'_, Postgres>, factura:&Factura 
) -> Result<i32, Error> {
	//let mut trans = db.begin().await?;

	const QRY:&str = 
		"insert into factura (id_front, cli_tipo_doc, cli_num_doc, tipo_fac, punto_venta, numero, fecha           , cae, venc_cae        , id_medio_pago) 
		Values							 ($1::UUID, $2          , $3         , $4      , $5         , $6    , $7::timestamptz , $8 , $9::timestamptz , $10          )
		Returning id_factura";

	let qry = query(QRY)
			/* 1*/.bind(&factura.id_front)
			/* 2*/.bind(factura.cliente.tipo_doc)
			/* 3*/.bind(factura.cliente.num_doc)
			/* 4*/.bind(factura.tipo_fac)
			/* 5*/.bind(factura.punto_venta)
			/* 6*/.bind(factura.numero)
			/* 7*/.bind(factura.fecha.format(FORMAT).to_string())
			/* 8*/.bind(factura.cae)
			/* 9*/.bind(factura.venc_cae.format(FORMAT).to_string())
			/*10*/.bind(factura.id_medio_pago)
		;
		
	let id_factura = qry.fetch_one(&mut **trans).await?.get(0);
	return Ok(id_factura);
}