use chrono::{DateTime, Utc};
use crate::CONF;

pub struct AfipAuth {
	pub xml:String,
	pub expir:DateTime<Utc>
}

impl AfipAuth {
	pub fn new(	
		token:String, sign:String, expir:DateTime<Utc>
	) ->Self {
		Self{
			xml: format!(
				"<ar:Auth>
					<ar:Token>{}</ar:Token>
					<ar:Sign>{}</ar:Sign>
					<ar:Cuit>{}</ar:Cuit>
				</ar:Auth>", token, sign, CONF.afip_com.cuit).to_string(),
			expir
		}
	}
}