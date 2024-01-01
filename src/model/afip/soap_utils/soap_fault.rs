use crate::model::afip::soap_utils::get_xml_tag;

#[derive(Debug)]
pub struct SoapFault {
	pub fault_code: Option<String>,
	pub fault_string: Option<String>, 
}

impl SoapFault {
	pub fn new(xml:&str) ->Self {
		Self{
			fault_string: Some(get_xml_tag(xml,"faultstring")),
			fault_code: Some(get_xml_tag(xml,"faultcode"))
		}
	}
}
