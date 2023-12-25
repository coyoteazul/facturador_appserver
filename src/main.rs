mod endpoints;
mod types; 
mod aux_func;
mod model;
use rocket_db_pools::{sqlx, Database};
use rocket::http::Status;
use model::afip::wsfev1::fe_dummy;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



#[launch]
async fn rocket() -> _ {
	let cli = reqwest::Client::builder()
		.user_agent("a")
		.build()
		.unwrap();

	let _res = model::afip::dummy::afip_dummy(&cli).await;
	//let res = model::afip::wsfev1::ports::ServiceSoap::fe_dummy(&'life0 self, fedummy_soap_in)
	

	

	rocket::build()
		.mount("/", routes![index])
		.mount("/", routes![endpoints::facturar::facturar])
		.mount("/", routes![endpoints::cliente::cliente_get])
}
