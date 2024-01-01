use chrono::{DateTime, Utc, Local};
use rocket::serde::{Deserialize, Serializer, Deserializer, de};

const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%:z";

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(
		date: &DateTime<Utc>,
		serializer: S,
) -> Result<S::Ok, S::Error>
where
		S: Serializer,
{
		let s = format!("{}", date.format(FORMAT));
		serializer.serialize_str(&s)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(
		deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
		D: Deserializer<'de>,
{
		let s = String::deserialize(deserializer)?;
		let dt = DateTime::parse_from_str(&s, FORMAT).map_err(de::Error::custom)?;
		return Ok(dt.with_timezone(&Utc));
		//Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}

pub fn date_to_yyyymmdd(
	fecha:DateTime<Utc>
) -> String {
	return format!("{}",fecha.with_timezone(&Local).format("%Y%m%d"));
}

pub fn utc_from_ts_str(txt: &str
)-> DateTime<Utc> {
	let x = DateTime::parse_from_str(txt, FORMAT).unwrap();
	return x.with_timezone(&Utc);
}

pub fn local_from_ts_str(txt: &str
)-> DateTime<Local> {
	let x = DateTime::parse_from_str(txt, FORMAT).unwrap();
	return x.with_timezone(&Local);
}
