use rocket::serde::{Deserialize, json::Json};



#[get("/cliente", data = "<input>")]
pub fn cliente_get(input: Json<ClienteGet>) { 
	println!("{:?}", input);

	
	
	//return input;
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct ClienteGet {
	pub tipo_doc:i8,
	pub num_doc:i32
}