use openssl::cms::CMSOptions;
use openssl::sign::Signer;
use openssl::rsa::Rsa;
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::hash::MessageDigest;
use openssl::x509::{X509Ref, X509};
use std::env;
use std::fs::File;
use std::io::Read;

fn example() {
	// Generate a keypair
	let keypair = Rsa::generate(2048).unwrap();
	let keypair = PKey::from_rsa(keypair).unwrap();

	let data = b"hello, world!";
	let data2 = b"hola, mundo!";

	// Sign the data
	let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
	signer.update(data).unwrap();
	signer.update(data2).unwrap();
	let signature = signer.sign_to_vec().unwrap();
}

fn test() {
	
	let cert_bytes = include_bytes!("./cert_afip_homo/claveHomoHernan.key");
	println!("cert_bytes{:?}", cert_bytes);
  
	let cert = X509::from_pem(cert_bytes).expect("failed to load cert.pem");
		println!("cert{:?}", cert);

	let signcert: Option<&X509Ref> = Some(cert.as_ref());

	
	
	//let pkey: Option<&PKeyRef<T>>
	//let certs: Option<&StackRef<X509>>
	//let data: Option<&[u8]>
	//let flags: CMSOptions

	//let a = openssl::cms::CmsContentInfo::sign(
	//	signcert, pkey, certs, data,flags
	//);
}

pub fn sign_with_cms (data:&str) ->String {
	const LEN_BEGIN:usize = "-----BEGIN CMS-----".len();
	const LEN_END:usize   = "-----END CMS-----
	".len();

	//Generar data firmada con PEM
	let binding = my_key();
  let pkey: Option<&PKeyRef<Private>> = Some(binding.as_ref());
	let signcert = my_cert();
	let data: Option<&[u8]> = Some(data.as_bytes());
	let flags = CMSOptions::empty();
	let mut pem = openssl::cms::CmsContentInfo::sign(
		Some(signcert.as_ref()), pkey, None, data,flags
	).unwrap().to_pem().unwrap();

	//remover cabezales
  pem.drain(pem.len() - LEN_END..);
	pem.drain(0..LEN_BEGIN);

  let pem = String::from_utf8(pem).unwrap();
	println!("mysign : firmado:{:?}",pem);
	return pem;
}

pub fn my_cert() ->X509 {
	const CERT_FILE:&str = "./afip_cert/Certificado.pem";
	let cert_path = env::current_exe().unwrap().parent().unwrap().join(CERT_FILE);
	println!("{}",cert_path.display());

	let mut buf:[u8;1212] = [0; 1212];
	let mut file = File::open(cert_path).unwrap();	
	let _ = file.read_exact(&mut buf).unwrap();
	
	return X509::from_pem(&buf).expect("failed to load my_cert");
}

pub fn my_key() ->PKey<Private> {
	const CERT_FILE:&str = "./afip_cert/MiClavePrivada.key";
	let cert_path = env::current_exe().unwrap().parent().unwrap().join(CERT_FILE);
	println!("{}",cert_path.display());

	let mut buf = Vec::new();
	let mut file = File::open(cert_path).unwrap();	
	file.read_to_end(&mut buf);

	/*let mut buf:[u8;1732] = [0; 1732];
	let mut file = File::open(cert_path).unwrap();	
	let _ = file.read_exact(&mut buf).unwrap();*/
	
	return PKey::private_key_from_pem(&buf).expect("failed to load my_key");
	//return X509::from_pem(&buf).expect("failed to load cert.pem");
}