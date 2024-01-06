use openssl::cms::CMSOptions;
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::x509::X509;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn sign_with_cms (data:&str) ->String {
	dbg!("Function call");
	const LEN_BEGIN:usize = "-----BEGIN CMS-----".len();
	const LEN_END:usize   = "-----END CMS-----
	".len();

	//Generar data firmada con PEM
	let binding = my_key_load();
  let pkey: Option<&PKeyRef<Private>> = Some(binding.as_ref());
	let signcert = my_cert_load();
	let data: Option<&[u8]> = Some(data.as_bytes());
	let flags = CMSOptions::empty();
	let mut pem = openssl::cms::CmsContentInfo::sign(
		Some(signcert.as_ref()), pkey, None, data,flags
	).unwrap().to_pem().unwrap();

	//remover cabezales
  pem.drain(pem.len() - LEN_END..);
	pem.drain(0..LEN_BEGIN);

  let pem = String::from_utf8(pem).unwrap();
	//dbg!("mysign : firmado:{:?}",pem);
	return pem;
}

pub fn cert_filepath() ->PathBuf {
	dbg!("Function call");
	const CERT_FILE:&str = "./afip_cert/Certificado.pem";
	return env::current_exe().unwrap().parent().unwrap().join(CERT_FILE);
}

fn my_cert_load() ->X509 {
	dbg!("Function call");
	let cert_path = cert_filepath();
	dbg!("{}",cert_path.display());

	let mut buf = Vec::new();
	let mut file = File::open(cert_path).unwrap();	
	let _ = file.read_to_end(&mut buf);
	
	return X509::from_pem(&buf).expect("failed to load my_cert");
}

pub fn key_filepath() ->PathBuf {
	dbg!("Function call");
	const CERT_FILE:&str = "./afip_cert/MiClavePrivada.key";
	return env::current_exe().unwrap().parent().unwrap().join(CERT_FILE);
}

fn my_key_load() ->PKey<Private> {
	dbg!("Function call");
	let cert_path = key_filepath();
	dbg!("{}",cert_path.display());

	let mut buf = Vec::new();
	let mut file = File::open(cert_path).unwrap();	
	let _ = file.read_to_end(&mut buf);
	
	return PKey::private_key_from_pem(&buf).expect("failed to load my_key");
}