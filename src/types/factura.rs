use chrono::{DateTime, Utc, TimeZone};
use rocket::serde::{Deserialize,Serialize};
use crate::types::factura_producto::FacturaProduct;
use crate::types::cliente::Cliente;
use crate::aux_func::time_serde;

#[derive(Deserialize, Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Factura {
	#[serde(default = "_default_i32")]
	pub id_factura	:i32,
	pub id_front    :String,
	pub cliente			:Cliente,
	pub tipo_fac		:i32,
	pub punto_venta	:i32,
	#[serde(default = "_default_i64")]
	pub numero			:i64,
	#[serde(with = "time_serde", default="_default_now")]
	pub fecha				:DateTime<Utc>,
	#[serde(default = "_default_i64")]
	pub cae					:i64,
	#[serde(with = "time_serde", default= "_default_dt")]
	pub venc_cae		:DateTime<Utc>,
	pub id_medio_pago	:i32,
	pub productos   :Vec<FacturaProduct>
}

impl Factura {
	pub fn total(&self) -> f64 {
		let mut retorno = 0.0;
		for i in &self.productos {
			retorno += i.valor;
		}
		return retorno;
	}
}
const fn _default_i32() -> i32 { -1 }
const fn _default_i64() -> i64 { -1 }
fn _default_dt() -> DateTime<Utc> {Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).single().unwrap()}
fn _default_now() -> DateTime<Utc> {Utc::now()}