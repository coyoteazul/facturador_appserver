use rocket_db_pools::sqlx::{query, Error, Transaction, Postgres, Row};
use crate::types::Factura;
const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%:z";

/** Registra la factura en la base de datos y actualiza 
 * el id_factura y numero en el objeto factura
 */
pub async fn db_factura_head_alta(
	trans: &mut Transaction<'_, Postgres>, 
	factura:&mut Factura
) -> Result<bool, Error> {
	//let mut trans = db.begin().await?;

	const QRY:&str = 
		"insert into factura 
			(id_front, cli_tipo_doc, cli_num_doc, tipo_fac, punto_venta, numero             , fecha           , id_medio_pago) Values
			($1::UUID, $2          , $3         , $4      , $5         , next_num_fac($4,$5), $6::timestamptz , $7           )
		Returning id_factura,numero";

	let qry = query(QRY)
			/* 1*/.bind(&factura.id_front)
			/* 2*/.bind(factura.cliente.tipo_doc)
			/* 3*/.bind(factura.cliente.num_doc)
			/* 4*/.bind(factura.tipo_fac)
			/* 5*/.bind(factura.punto_venta)
			/* 6*/.bind(factura.fecha.format(FORMAT).to_string())
			/* 7*/.bind(factura.id_medio_pago)
		;
		
	let row = qry.fetch_one(&mut **trans).await?;
	factura.id_factura = row.get("id_factura");
	factura.numero = row.get("numero");
	return Ok(true);
}