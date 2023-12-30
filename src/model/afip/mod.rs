pub mod wsfev1;
mod soap_utils;


pub use soap_utils::afip_signin::afip_signin;
pub use soap_utils::{cert_filepath, key_filepath};