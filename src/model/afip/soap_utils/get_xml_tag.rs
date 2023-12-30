pub fn get_xml_tag(xml: &str, tag:&str) -> String {
	let start_tag = format!("<{tag}>");
	let end_tag   = format!("</{tag}>");

	if let Some(start_index) = xml.find(&start_tag) {
			if let Some(end_index) = xml.find(&end_tag) {
					let body_content = &xml[start_index + start_tag.len()..end_index];
					return body_content.to_string();
			}
	}

	String::from("")
}