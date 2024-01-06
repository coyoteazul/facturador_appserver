use rocket_db_pools::sqlx::Error;
use rocket_db_pools::{Connection, sqlx};
use rocket_db_pools::sqlx::Row;
use crate::Db;
use crate::types::ProductoGrupo;

pub async fn db_producto_grupo_get_all(
	db: &mut Connection<Db>
) -> Result<Vec<ProductoGrupo>, Error> {
	const QRY:&str = 
	"select id_producto_grupo, nombre from producto_grupo pg
	where exists(select from producto p where p.id_producto_grupo= pg.id_producto_grupo)";

	let rows = sqlx::query(QRY).fetch_all(&mut ***db).await.unwrap();
	
	let mut retorno:Vec<ProductoGrupo> = Vec::<ProductoGrupo>::with_capacity(rows.len()); 
	for rec in rows {
		retorno.push(ProductoGrupo{
			id_producto_grupo: rec.get("id_producto_grupo"),
			nombre: 				   rec.get("nombre")
		})
			
	};

	return Ok(retorno);
}