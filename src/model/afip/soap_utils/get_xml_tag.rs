pub fn get_xml_tag(xml: &str, tag:&str) -> String {
	dbg!("Function call");
	let start_tag = format!("<{tag}>");
	let end_tag   = format!("</{tag}>");

	if let Some(start_index) = xml.find(&start_tag) {
		let body_content = &xml[start_index + start_tag.len()..];
			if let Some(end_index) = body_content.find(&end_tag) {
					let body_content = &body_content[0..end_index];
					return body_content.to_string();
			}
	}

	//volver a intentar si no encuentra con < y >
	let start_tag = format!("&lt;{tag}&gt;");
	let end_tag   = format!("&lt;/{tag}&gt;");
	if let Some(start_index) = xml.find(&start_tag) {
		let body_content = &xml[start_index + start_tag.len()..];
			if let Some(end_index) = body_content.find(&end_tag) {
					let body_content = &body_content[0..end_index];
					return body_content.to_string();
			}
	}

	String::from("")
}