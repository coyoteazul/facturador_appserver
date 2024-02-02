use chrono::{Utc, DateTime};

pub fn arma_login_ticket_request_xml(
	webservice:&str,req_date: DateTime<Utc>,exp_date: DateTime<Utc>
) -> String {
	dbg!("Function call");
	let gen_time = req_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
	let exp_time = exp_date.format("%Y-%m-%dT%H:%M:%S%:z").to_string();
	let uniqueid = req_date.timestamp().to_string();

	let login_ticket_request_xml = format!(
	r#"<loginTicketRequest version="1.0">
		<header>
			<uniqueId>{uniqueid}</uniqueId>
			<generationTime>{gen_time}</generationTime>
			<expirationTime>{exp_time}</expirationTime>
		</header>
		<service>{webservice}</service>
	</loginTicketRequest>"#
	);
	return login_ticket_request_xml;
}

