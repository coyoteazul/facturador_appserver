mod afip_request;
mod arma_login_ticket_request;
mod cms_sign;
pub mod afip_signin;
mod get_xml_tag;
mod soap_fault;

pub use afip_request::afip_post;
pub use afip_signin::afip_signin;
pub use get_xml_tag::get_xml_tag;
pub use cms_sign::{cert_filepath, key_filepath};