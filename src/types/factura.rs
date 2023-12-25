use chrono::{DateTime, Utc};
use rocket::serde::{Deserialize};
use crate::types::factura_producto::FacturaProduct;
use crate::types::cliente::Cliente;
use crate::aux_func::time_serde;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Factura {
	pub id					:String,
	pub cliente			:Cliente,
	pub tipo_fac		:i8,
	pub punto_venta	:i16,
	pub numero			:i32,
	//#[serde(deserialize_with = "deserialize_datetime_utc")]
	#[serde(with = "time_serde")]
	pub fecha				:DateTime<Utc>,
	pub cae					:i32,
	#[serde(with = "time_serde")]
	pub venc_cae		:DateTime<Utc>,
	pub medio_pago	:String,
	pub productos   :Vec<FacturaProduct>
}


