use std::process::Command;
use chrono::Local;

use escposify::{img::Image, printer::Printer};
use image::Luma;
use qrcode::QrCode;
use rocket::serde::{Serialize, json};
use base64::{engine::general_purpose, Engine as _};
use tempfile::NamedTempFile;

use crate::{types::{Factura, Cliente, Producto}, CONF, conf::AfipCom};


pub fn print_factura(factura:&Factura) {
	let device_file =  NamedTempFile::new().unwrap();

	//let device_file = File::options().append(true).open("D:\\TMP\\testfile").unwrap();
	let file = escposify::device::File::from(device_file.as_file());
	let mut printer = Printer::new(file, None, None);

	let mut pr = printer
        .chain_align("ct").unwrap();

	for lin in text_make(&factura){
		pr = pr.chain_text(&lin).unwrap()
	}
				
	if factura.tipo_fac != 0 {
		pr = pr.chain_bit_image(&qr_make(&factura), Some("d24")).unwrap()
	}

	let _ = pr.chain_feed(20).unwrap().flush();	

	let source_path = &device_file.path().display().to_string();
	let destination_path = &CONF.entorno.impresora;
	let _status = Command::new("cmd")
			.args(&["/C", "copy", source_path, destination_path])
			.status()
			.expect("Failed to execute command");
}

fn qr_make(factura: &Factura) -> Image {
	let json_fac = FacJson {
    ver: 				1,
    fecha: 			factura.fecha.format("%Y-%m-%d").to_string(),
    cuit: 			CONF.afip_com.cuit,
    ptoVta: 		factura.punto_venta,
    tipoCmp: 		factura.tipo_fac,
    nroCmp: 		factura.numero,
    importe: 		factura.total(),
    moneda: 		"PES".to_string(),
    ctz:				1.0,
    tipoDocRec: factura.cliente.tipo_doc,
    nroDocRec: 	factura.cliente.num_doc,
    tipoCodAut: "E".to_string(),
    codAut: 		factura.cae,
	};
	let json_str = json::to_string(&json_fac).unwrap();
	let base64 = general_purpose::STANDARD.encode(json_str);

	let code = QrCode::new(format!("https://www.afip.gob.ar/fe/qr/?{base64}")).unwrap();
	// Render the bits into an image.
	let a = code.render::<Luma<u8>>().build();
	
	return Image::from(image::DynamicImage::ImageLuma8(a)
	.resize(400, 400, image::imageops::FilterType::Nearest));
}

#[allow(non_snake_case)]
#[derive( Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct FacJson {
ver:i32,
fecha:String,
cuit:i64,
ptoVta:i32,
tipoCmp:i32,
nroCmp:i64,
importe:f64,
moneda:String,
ctz:f64,
tipoDocRec:i32,
nroDocRec:i64,
tipoCodAut:String,
codAut:i64}

fn text_make(factura: &Factura
) -> Vec<String>  {
	const MAX_SIZE_LINE:usize = 33;
	let mut ret = Vec::<String>::with_capacity(24+&factura.productos.len());	
	let AfipCom{direccion,inicio_actividad,iva_responsable,nombre_fantasia, cuit, iibb_num}	= &CONF.afip_com;
	let Factura{venc_cae,cae, fecha, punto_venta, numero,..}	= factura;
	let Cliente{nombre, num_doc, tipo_doc: _} = &factura.cliente;
	let cuit_str = cuit.to_string();
	let ib_str = iibb_num.to_string();
	let cli_doc_str = num_doc.to_string();
	let fecha_str = fecha.with_timezone(&Local).format("%d/%m/%Y %H:%M").to_string();
	let cae_str = cae.to_string();
	let cae_venc_str = venc_cae.format("%d/%m/%Y").to_string();
	let comprobante = format!("{:05}-{:08}", punto_venta,  numero);
	let total_str = format!("{:.2}", factura.total());
	let t_str_len = total_str.chars().count()+1;	
	let medio = &factura.medio_pago.nombre;
	let doc_leye = if factura.tipo_fac == 0 {"Documento no valido como factura"} else {" "};
	let tipodoc = if factura.tipo_fac == 0 {"Presupuesto"} else {"Factura C"};

	let cabezal=format!(
"{doc_leye}
   {nombre_fantasia}
C.U.I.T: {cuit_str}
Ing.Brutos: {ib_str}
{direccion}
Inicio de Act.: {inicio_actividad}
IVA {iva_responsable}
    Original
{tipodoc} {comprobante}
Fecha: {fecha_str}
Cliente: {nombre}
C.U.I.T: {cli_doc_str}
IVA Consumidor Final
\r\n
 Descripcion             Total");
	
let footer=format!(
"Total: ${total_str}
\n\n
Recibimos: {medio}

ORIENTACION AL CONSUMIDOR 
PROV. DE BUENOS AIRES
TEL: 0800-222-9042");

let cae_prnt=format!("
CAE:{cae_str}
VTO CAE: {cae_venc_str}");

	/*for i in textwrap::wrap(&cabezal, MAX_SIZE_LINE) {
		ret.push(i.to_string());
	}*/
		ret.push(textwrap::fill(&cabezal, MAX_SIZE_LINE));

	for linea in &factura.productos {
		let mut count = 0;
		let Producto{nombre, unidad_medida, ..} = &linea.producto;
		let concepto = format!("{:.2}{} de: {}",linea.cantidad, unidad_medida, nombre );
		
		let wrap = textwrap::wrap(&concepto, MAX_SIZE_LINE - t_str_len);

		for str in wrap {
			if count == 0 {
				let largo_conc = str.chars().count();
				let valor = format!("${:.2}", linea.valor);
				let largo_val = valor.chars().count();
				let espacios = " ".repeat(MAX_SIZE_LINE - largo_conc - largo_val -2);
				
				ret.push(format!("{str}{espacios}{valor}").to_string());
			} else {
				let lin =str.to_string();
				if lin.len()>0 {
					ret.push(lin);
				}
			};
			count += 1;
		}
	}

	/*for i in textwrap::wrap(&footer, MAX_SIZE_LINE) {
		ret.push(i.to_string());
	}*/
	ret.push(textwrap::fill(&footer, MAX_SIZE_LINE));

	if factura.tipo_fac != 0 {
		ret.push(textwrap::fill(&cae_prnt, MAX_SIZE_LINE));
	}

	return ret;
}

