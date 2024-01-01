use reqwest::Client;
use rocket::{serde::{json::{Json, self}}, http::{Status, ContentType}, State};
use rocket_db_pools::{Connection, sqlx::error::DatabaseError};
use rocket_db_pools::sqlx::error::ErrorKind::UniqueViolation;
use crate::{types::Factura, model::{propio::factura::{db_factura_alta}, afip::wsfev1::afip_fe_cae_solicitar}, Db};

#[post("/facturar", data = "<input>")]
pub async fn facturar(
	mut db: Connection<Db>, 
	input: Json<Factura>, 
	req_cli: &State<Client>
) ->(Status, (ContentType, String)) { 
	dbg!("facturar input{:?}", &input);
	let mut factura = input.0;

	//Alta de head e items en la base
	let res = db_factura_alta(&mut db, &mut factura).await;

	match res {
		Ok(estado) => {
			if estado {
				let soli_res = afip_fe_cae_solicitar(&req_cli, &mut factura, &mut db).await;
				match soli_res {
					Ok(status) => {
						if status.0 {
							return (Status::Ok, (ContentType::JSON, 
								json::to_string(&factura).unwrap()));
						} else {
							return (Status::InternalServerError, (ContentType::JSON, 
								format!("{}", status.1)));
						}
					}
					Err(e) => {
						return (Status::InternalServerError, (ContentType::JSON, 
							format!("{:?}", e)));
					}
				}
				

			} else {
				return (Status::InternalServerError, (ContentType::JSON, 
					String::new()));
			}
		}
		Err(e) => {
			let a: &dyn DatabaseError = e.as_database_error().unwrap();
			dbg!("facturar err:{:?}", a);
			match a.kind() {
				UniqueViolation =>{
					return (Status::Conflict, (ContentType::JSON, 
						format!("{:?}", a)));
				}
				_ => {
					return (Status::InternalServerError, (ContentType::JSON, 
						format!("{:?}", a)));
				}
			}
		}
	}
}