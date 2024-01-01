mod factura_head;
mod factura_producto;
mod factura;
mod factura_transmision;

pub use factura::{db_factura_alta, db_factura_set_cae};
pub use factura_transmision::*;