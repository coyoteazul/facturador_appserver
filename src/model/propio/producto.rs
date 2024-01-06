use rocket_db_pools::sqlx::Error;
use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::Row;
use crate::Db;
use crate::types:: Producto;

pub async fn db_producto_get_all(
	db: &mut Connection<Db>
) -> Result<Vec<Producto>, Error> {
	const QRY:&str = 
	"select p.*, pg.nombre as producto_grupo
	from producto p
	inner join producto_grupo pg
	using (id_producto_grupo)";

	let rows = sqlx::query(QRY).fetch_all(&mut ***db).await.unwrap();
	
	let mut retorno:Vec<Producto> = Vec::<Producto>::with_capacity(rows.len()); 
	for rec in rows {
		retorno.push(Producto{
			id_producto: 		rec.get("id_producto"),
			nombre: 				rec.get("nombre"),
			unidad_medida:	rec.get("unidad_medida"),
			producto_grupo: rec.get("producto_grupo"),
			precio: 				rec.get("precio")
		})
			
	};

	return Ok(retorno);
}