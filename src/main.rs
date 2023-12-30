mod endpoints;
mod types; 
mod aux_func;
mod model;
use rocket_db_pools::{sqlx, Database};
use rocket::{launch, routes};

#[derive(Database)]
#[database("my_postgres_db")]
pub struct Db(sqlx::PgPool);

#[macro_use] extern crate rocket;
#[launch]
async fn rocket() -> _ {
	let cli = reqwest::Client::builder()
		.user_agent("a")
		.build()
		.unwrap();

	rocket::build()
		.manage(cli)
		.attach(Db::init())
		.mount("/", routes![endpoints::facturar::facturar])
		.mount("/", routes![endpoints::cliente::cliente_get])
		.mount("/", routes![endpoints::cliente::cliente_alta])
}
