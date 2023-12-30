use chrono::{DateTime, Utc};
use rocket::serde::Deserialize;
use crate::types::factura_producto::FacturaProduct;
use crate::types::cliente::Cliente;
use crate::aux_func::time_serde;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Factura {
	pub id_front    :String,
	pub cliente			:Cliente,
	pub tipo_fac		:i32,
	pub punto_venta	:i32,
	pub numero			:i64,
	#[serde(with = "time_serde")]
	pub fecha				:DateTime<Utc>,
	pub cae					:i64,
	#[serde(with = "time_serde")]
	pub venc_cae		:DateTime<Utc>,
	pub id_medio_pago	:i32,
	pub productos   :Vec<FacturaProduct>
}

impl Factura {
	pub fn total(&self) -> f64 {
		let mut retorno = 0.0;
		for i in self.productos {
			retorno += i.valor;
		}
		return retorno;
	}
}


