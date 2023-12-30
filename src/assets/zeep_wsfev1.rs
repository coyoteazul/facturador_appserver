//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.6
//!

            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde_derive::{YaSerialize, YaDeserialize};
            use std::io::{Read, Write};
            use log::{warn, debug};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Fault",
	namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
	prefix = "soapenv",
)]
pub struct SoapFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>, 
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>, 
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitarSoapIn",
)]
pub struct FecaesolicitarSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::Fecaesolicitar, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitarSoapOut",
)]
pub struct FecaesolicitarSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaesolicitarResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompTotXRequestSoapIn",
)]
pub struct FecompTotXRequestSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FecompTotXRequest, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompTotXRequestSoapOut",
)]
pub struct FecompTotXRequestSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecompTotXRequestResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDummySoapIn",
)]
pub struct FedummySoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::Fedummy, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDummySoapOut",
)]
pub struct FedummySoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FedummyResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompUltimoAutorizadoSoapIn",
)]
pub struct FecompUltimoAutorizadoSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FecompUltimoAutorizado, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompUltimoAutorizadoSoapOut",
)]
pub struct FecompUltimoAutorizadoSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecompUltimoAutorizadoResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsultarSoapIn",
)]
pub struct FecompConsultarSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FecompConsultar, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsultarSoapOut",
)]
pub struct FecompConsultarSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecompConsultarResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEARegInformativoSoapIn",
)]
pub struct FecaearegInformativoSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaearegInformativo, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEARegInformativoSoapOut",
)]
pub struct FecaearegInformativoSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaearegInformativoResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASolicitarSoapIn",
)]
pub struct FecaeasolicitarSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::Fecaeasolicitar, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASolicitarSoapOut",
)]
pub struct FecaeasolicitarSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaeasolicitarResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoConsultarSoapIn",
)]
pub struct FecaeasinMovimientoConsultarSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaeasinMovimientoConsultar, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoConsultarSoapOut",
)]
pub struct FecaeasinMovimientoConsultarSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaeasinMovimientoConsultarResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoInformarSoapIn",
)]
pub struct FecaeasinMovimientoInformarSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaeasinMovimientoInformar, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoInformarSoapOut",
)]
pub struct FecaeasinMovimientoInformarSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaeasinMovimientoInformarResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAConsultarSoapIn",
)]
pub struct FecaeaconsultarSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::Fecaeaconsultar, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAConsultarSoapOut",
)]
pub struct FecaeaconsultarSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FecaeaconsultarResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetCotizacionSoapIn",
)]
pub struct FeparamGetCotizacionSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetCotizacion, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetCotizacionSoapOut",
)]
pub struct FeparamGetCotizacionSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetCotizacionResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposTributosSoapIn",
)]
pub struct FeparamGetTiposTributosSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposTributos, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposTributosSoapOut",
)]
pub struct FeparamGetTiposTributosSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposTributosResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposMonedasSoapIn",
)]
pub struct FeparamGetTiposMonedasSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposMonedas, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposMonedasSoapOut",
)]
pub struct FeparamGetTiposMonedasSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposMonedasResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposIvaSoapIn",
)]
pub struct FeparamGetTiposIvaSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposIva, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposIvaSoapOut",
)]
pub struct FeparamGetTiposIvaSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposIvaResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposOpcionalSoapIn",
)]
pub struct FeparamGetTiposOpcionalSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposOpcional, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposOpcionalSoapOut",
)]
pub struct FeparamGetTiposOpcionalSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposOpcionalResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposConceptoSoapIn",
)]
pub struct FeparamGetTiposConceptoSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposConcepto, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposConceptoSoapOut",
)]
pub struct FeparamGetTiposConceptoSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposConceptoResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetPtosVentaSoapIn",
)]
pub struct FeparamGetPtosVentaSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetPtosVenta, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetPtosVentaSoapOut",
)]
pub struct FeparamGetPtosVentaSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetPtosVentaResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposCbteSoapIn",
)]
pub struct FeparamGetTiposCbteSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposCbte, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposCbteSoapOut",
)]
pub struct FeparamGetTiposCbteSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposCbteResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposDocSoapIn",
)]
pub struct FeparamGetTiposDocSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposDoc, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposDocSoapOut",
)]
pub struct FeparamGetTiposDocSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposDocResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposPaisesSoapIn",
)]
pub struct FeparamGetTiposPaisesSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposPaises, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposPaisesSoapOut",
)]
pub struct FeparamGetTiposPaisesSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetTiposPaisesResponse, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetActividadesSoapIn",
)]
pub struct FeparamGetActividadesSoapIn {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetActividades, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetActividadesSoapOut",
)]
pub struct FeparamGetActividadesSoapOut {
	#[yaserde(flatten, default)]
	pub parameters: types::FeparamGetActividadesResponse, 
}
}

pub mod types {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitar",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct Fecaesolicitar {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "FeCAEReq", prefix = "wsfev1", default)]
	pub fe_cae_req: Option<Fecaerequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEAuthRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FeauthRequest {
	#[yaserde(rename = "Token", prefix = "wsfev1", default)]
	pub token: Option<String>, 
	#[yaserde(rename = "Sign", prefix = "wsfev1", default)]
	pub sign: Option<String>, 
	#[yaserde(rename = "Cuit", prefix = "wsfev1", default)]
	pub cuit: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAERequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Fecaerequest {
	#[yaserde(rename = "FeCabReq", prefix = "wsfev1", default)]
	pub fe_cab_req: Option<FecaecabRequest>, 
	#[yaserde(rename = "FeDetReq", prefix = "wsfev1", default)]
	pub fe_det_req: Option<ArrayOfFECAEDetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAECabRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaecabRequest {
	#[yaserde(flatten, default)]
	pub fecab_request: FecabRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECabRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecabRequest {
	#[yaserde(rename = "CantReg", prefix = "wsfev1", default)]
	pub cant_reg: i32, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEDetRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfFECAEDetRequest {
	#[yaserde(rename = "FECAEDetRequest", prefix = "wsfev1", default)]
	pub fecae_det_request: Vec<FecaedetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEDetRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaedetRequest {
	#[yaserde(flatten, default)]
	pub fedet_request: FedetRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDetRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FedetRequest {
	#[yaserde(rename = "Concepto", prefix = "wsfev1", default)]
	pub concepto: i32, 
	#[yaserde(rename = "DocTipo", prefix = "wsfev1", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "wsfev1", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "CbteDesde", prefix = "wsfev1", default)]
	pub cbte_desde: i64, 
	#[yaserde(rename = "CbteHasta", prefix = "wsfev1", default)]
	pub cbte_hasta: i64, 
	#[yaserde(rename = "CbteFch", prefix = "wsfev1", default)]
	pub cbte_fch: Option<String>, 
	#[yaserde(rename = "ImpTotal", prefix = "wsfev1", default)]
	pub imp_total: f64, 
	#[yaserde(rename = "ImpTotConc", prefix = "wsfev1", default)]
	pub imp_tot_conc: f64, 
	#[yaserde(rename = "ImpNeto", prefix = "wsfev1", default)]
	pub imp_neto: f64, 
	#[yaserde(rename = "ImpOpEx", prefix = "wsfev1", default)]
	pub imp_op_ex: f64, 
	#[yaserde(rename = "ImpTrib", prefix = "wsfev1", default)]
	pub imp_trib: f64, 
	#[yaserde(rename = "ImpIVA", prefix = "wsfev1", default)]
	pub imp_iva: f64, 
	#[yaserde(rename = "FchServDesde", prefix = "wsfev1", default)]
	pub fch_serv_desde: Option<String>, 
	#[yaserde(rename = "FchServHasta", prefix = "wsfev1", default)]
	pub fch_serv_hasta: Option<String>, 
	#[yaserde(rename = "FchVtoPago", prefix = "wsfev1", default)]
	pub fch_vto_pago: Option<String>, 
	#[yaserde(rename = "MonId", prefix = "wsfev1", default)]
	pub mon_id: Option<String>, 
	#[yaserde(rename = "MonCotiz", prefix = "wsfev1", default)]
	pub mon_cotiz: f64, 
	#[yaserde(rename = "CbtesAsoc", prefix = "wsfev1", default)]
	pub cbtes_asoc: Option<ArrayOfCbteAsoc>, 
	#[yaserde(rename = "Tributos", prefix = "wsfev1", default)]
	pub tributos: Option<ArrayOfTributo>, 
	#[yaserde(rename = "Iva", prefix = "wsfev1", default)]
	pub iva: Option<ArrayOfAlicIva>, 
	#[yaserde(rename = "Opcionales", prefix = "wsfev1", default)]
	pub opcionales: Option<ArrayOfOpcional>, 
	#[yaserde(rename = "Compradores", prefix = "wsfev1", default)]
	pub compradores: Option<ArrayOfComprador>, 
	#[yaserde(rename = "PeriodoAsoc", prefix = "wsfev1", default)]
	pub periodo_asoc: Option<Periodo>, 
	#[yaserde(rename = "Actividades", prefix = "wsfev1", default)]
	pub actividades: Option<ArrayOfActividad>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCbteAsoc",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfCbteAsoc {
	#[yaserde(rename = "CbteAsoc", prefix = "wsfev1", default)]
	pub cbte_asoc: Vec<CbteAsoc>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CbteAsoc",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct CbteAsoc {
	#[yaserde(rename = "Tipo", prefix = "wsfev1", default)]
	pub tipo: i32, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "Nro", prefix = "wsfev1", default)]
	pub nro: i64, 
	#[yaserde(rename = "Cuit", prefix = "wsfev1", default)]
	pub cuit: Option<String>, 
	#[yaserde(rename = "CbteFch", prefix = "wsfev1", default)]
	pub cbte_fch: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTributo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfTributo {
	#[yaserde(rename = "Tributo", prefix = "wsfev1", default)]
	pub tributo: Vec<Tributo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Tributo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Tributo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i16, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "BaseImp", prefix = "wsfev1", default)]
	pub base_imp: f64, 
	#[yaserde(rename = "Alic", prefix = "wsfev1", default)]
	pub alic: f64, 
	#[yaserde(rename = "Importe", prefix = "wsfev1", default)]
	pub importe: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAlicIva",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfAlicIva {
	#[yaserde(rename = "AlicIva", prefix = "wsfev1", default)]
	pub alic_iva: Vec<AlicIva>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlicIva",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct AlicIva {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i32, 
	#[yaserde(rename = "BaseImp", prefix = "wsfev1", default)]
	pub base_imp: f64, 
	#[yaserde(rename = "Importe", prefix = "wsfev1", default)]
	pub importe: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfOpcional",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfOpcional {
	#[yaserde(rename = "Opcional", prefix = "wsfev1", default)]
	pub opcional: Vec<Opcional>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Opcional",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Opcional {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Valor", prefix = "wsfev1", default)]
	pub valor: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfComprador",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfComprador {
	#[yaserde(rename = "Comprador", prefix = "wsfev1", default)]
	pub comprador: Vec<Comprador>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Comprador",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Comprador {
	#[yaserde(rename = "DocTipo", prefix = "wsfev1", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "wsfev1", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "Porcentaje", prefix = "wsfev1", default)]
	pub porcentaje: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Periodo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Periodo {
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfActividad",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfActividad {
	#[yaserde(rename = "Actividad", prefix = "wsfev1", default)]
	pub actividad: Vec<Actividad>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Actividad",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Actividad {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAESolicitarResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaesolicitarResponse {
	#[yaserde(rename = "FECAESolicitarResult", prefix = "wsfev1", default)]
	pub fecae_solicitar_result: Option<Fecaeresponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Fecaeresponse {
	#[yaserde(rename = "FeCabResp", prefix = "wsfev1", default)]
	pub fe_cab_resp: Option<FecaecabResponse>, 
	#[yaserde(rename = "FeDetResp", prefix = "wsfev1", default)]
	pub fe_det_resp: Option<ArrayOfFECAEDetResponse>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAECabResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaecabResponse {
	#[yaserde(flatten, default)]
	pub fecab_response: FecabResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECabResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecabResponse {
	#[yaserde(rename = "Cuit", prefix = "wsfev1", default)]
	pub cuit: i64, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
	#[yaserde(rename = "FchProceso", prefix = "wsfev1", default)]
	pub fch_proceso: Option<String>, 
	#[yaserde(rename = "CantReg", prefix = "wsfev1", default)]
	pub cant_reg: i32, 
	#[yaserde(rename = "Resultado", prefix = "wsfev1", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Reproceso", prefix = "wsfev1", default)]
	pub reproceso: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEDetResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfFECAEDetResponse {
	#[yaserde(rename = "FECAEDetResponse", prefix = "wsfev1", default)]
	pub fecae_det_response: Vec<FecaedetResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEDetResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaedetResponse {
	#[yaserde(flatten, default)]
	pub fedet_response: FedetResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "CAE", prefix = "wsfev1", default)]
	pub cae: Option<String>, 
	#[yaserde(rename = "CAEFchVto", prefix = "wsfev1", default)]
	pub cae_fch_vto: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDetResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FedetResponse {
	#[yaserde(rename = "Concepto", prefix = "wsfev1", default)]
	pub concepto: i32, 
	#[yaserde(rename = "DocTipo", prefix = "wsfev1", default)]
	pub doc_tipo: i32, 
	#[yaserde(rename = "DocNro", prefix = "wsfev1", default)]
	pub doc_nro: i64, 
	#[yaserde(rename = "CbteDesde", prefix = "wsfev1", default)]
	pub cbte_desde: i64, 
	#[yaserde(rename = "CbteHasta", prefix = "wsfev1", default)]
	pub cbte_hasta: i64, 
	#[yaserde(rename = "CbteFch", prefix = "wsfev1", default)]
	pub cbte_fch: Option<String>, 
	#[yaserde(rename = "Resultado", prefix = "wsfev1", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Observaciones", prefix = "wsfev1", default)]
	pub observaciones: Option<ArrayOfObs>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfObs",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfObs {
	#[yaserde(rename = "Obs", prefix = "wsfev1", default)]
	pub obs: Vec<Obs>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Obs",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Obs {
	#[yaserde(rename = "Code", prefix = "wsfev1", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "wsfev1", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEvt",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfEvt {
	#[yaserde(rename = "Evt", prefix = "wsfev1", default)]
	pub evt: Vec<Evt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Evt",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Evt {
	#[yaserde(rename = "Code", prefix = "wsfev1", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "wsfev1", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfErr",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfErr {
	#[yaserde(rename = "Err", prefix = "wsfev1", default)]
	pub err: Vec<Err>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Err",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Err {
	#[yaserde(rename = "Code", prefix = "wsfev1", default)]
	pub code: i32, 
	#[yaserde(rename = "Msg", prefix = "wsfev1", default)]
	pub msg: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompTotXRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecompTotXRequest {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompTotXRequestResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecompTotXRequestResponse {
	#[yaserde(rename = "FECompTotXRequestResult", prefix = "wsfev1", default)]
	pub fe_comp_tot_x_request_result: Option<FeregXReqResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FERegXReqResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FeregXReqResponse {
	#[yaserde(rename = "RegXReq", prefix = "wsfev1", default)]
	pub reg_x_req: i32, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDummy",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct Fedummy {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEDummyResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FedummyResponse {
	#[yaserde(rename = "FEDummyResult", prefix = "wsfev1", default)]
	pub fe_dummy_result: Option<DummyResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DummyResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct DummyResponse {
	#[yaserde(rename = "AppServer", prefix = "wsfev1", default)]
	pub app_server: Option<String>, 
	#[yaserde(rename = "DbServer", prefix = "wsfev1", default)]
	pub db_server: Option<String>, 
	#[yaserde(rename = "AuthServer", prefix = "wsfev1", default)]
	pub auth_server: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompUltimoAutorizado",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecompUltimoAutorizado {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompUltimoAutorizadoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecompUltimoAutorizadoResponse {
	#[yaserde(rename = "FECompUltimoAutorizadoResult", prefix = "wsfev1", default)]
	pub fe_comp_ultimo_autorizado_result: Option<FerecuperaLastCbteResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FERecuperaLastCbteResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FerecuperaLastCbteResponse {
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
	#[yaserde(rename = "CbteNro", prefix = "wsfev1", default)]
	pub cbte_nro: i32, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsultar",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecompConsultar {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "FeCompConsReq", prefix = "wsfev1", default)]
	pub fe_comp_cons_req: Option<FecompConsultaReq>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsultaReq",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecompConsultaReq {
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
	#[yaserde(rename = "CbteNro", prefix = "wsfev1", default)]
	pub cbte_nro: i64, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsultarResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecompConsultarResponse {
	#[yaserde(rename = "FECompConsultarResult", prefix = "wsfev1", default)]
	pub fe_comp_consultar_result: Option<FecompConsultaResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsultaResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecompConsultaResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<FecompConsResponse>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECompConsResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecompConsResponse {
	#[yaserde(flatten, default)]
	pub fecaedet_request: FecaedetRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Resultado", prefix = "wsfev1", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "CodAutorizacion", prefix = "wsfev1", default)]
	pub cod_autorizacion: Option<String>, 
	#[yaserde(rename = "EmisionTipo", prefix = "wsfev1", default)]
	pub emision_tipo: Option<String>, 
	#[yaserde(rename = "FchVto", prefix = "wsfev1", default)]
	pub fch_vto: Option<String>, 
	#[yaserde(rename = "FchProceso", prefix = "wsfev1", default)]
	pub fch_proceso: Option<String>, 
	#[yaserde(rename = "Observaciones", prefix = "wsfev1", default)]
	pub observaciones: Option<ArrayOfObs>, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEARegInformativo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaearegInformativo {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "FeCAEARegInfReq", prefix = "wsfev1", default)]
	pub fe_caea_reg_inf_req: Option<Fecaearequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEARequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Fecaearequest {
	#[yaserde(rename = "FeCabReq", prefix = "wsfev1", default)]
	pub fe_cab_req: Option<FecaeacabRequest>, 
	#[yaserde(rename = "FeDetReq", prefix = "wsfev1", default)]
	pub fe_det_req: Option<ArrayOfFECAEADetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEACabRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeacabRequest {
	#[yaserde(flatten, default)]
	pub fecab_request: FecabRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEADetRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfFECAEADetRequest {
	#[yaserde(rename = "FECAEADetRequest", prefix = "wsfev1", default)]
	pub fecaea_det_request: Vec<FecaeadetRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEADetRequest",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeadetRequest {
	#[yaserde(flatten, default)]
	pub fedet_request: FedetRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "CAEA", prefix = "wsfev1", default)]
	pub caea: Option<String>, 
	#[yaserde(rename = "CbteFchHsGen", prefix = "wsfev1", default)]
	pub cbte_fch_hs_gen: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEARegInformativoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaearegInformativoResponse {
	#[yaserde(rename = "FECAEARegInformativoResult", prefix = "wsfev1", default)]
	pub fecaea_reg_informativo_result: Option<Fecaearesponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Fecaearesponse {
	#[yaserde(rename = "FeCabResp", prefix = "wsfev1", default)]
	pub fe_cab_resp: Option<FecaeacabResponse>, 
	#[yaserde(rename = "FeDetResp", prefix = "wsfev1", default)]
	pub fe_det_resp: Option<ArrayOfFECAEADetResponse>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEACabResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeacabResponse {
	#[yaserde(flatten, default)]
	pub fecab_response: FecabResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEADetResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfFECAEADetResponse {
	#[yaserde(rename = "FECAEADetResponse", prefix = "wsfev1", default)]
	pub fecaea_det_response: Vec<FecaeadetResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEADetResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeadetResponse {
	#[yaserde(flatten, default)]
	pub fedet_response: FedetResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "CAEA", prefix = "wsfev1", default)]
	pub caea: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASolicitar",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct Fecaeasolicitar {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "Periodo", prefix = "wsfev1", default)]
	pub periodo: i32, 
	#[yaserde(rename = "Orden", prefix = "wsfev1", default)]
	pub orden: i16, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASolicitarResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaeasolicitarResponse {
	#[yaserde(rename = "FECAEASolicitarResult", prefix = "wsfev1", default)]
	pub fecaea_solicitar_result: Option<FecaeagetResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAGetResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeagetResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<Fecaeaget>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAGet",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Fecaeaget {
	#[yaserde(rename = "CAEA", prefix = "wsfev1", default)]
	pub caea: Option<String>, 
	#[yaserde(rename = "Periodo", prefix = "wsfev1", default)]
	pub periodo: i32, 
	#[yaserde(rename = "Orden", prefix = "wsfev1", default)]
	pub orden: i16, 
	#[yaserde(rename = "FchVigDesde", prefix = "wsfev1", default)]
	pub fch_vig_desde: Option<String>, 
	#[yaserde(rename = "FchVigHasta", prefix = "wsfev1", default)]
	pub fch_vig_hasta: Option<String>, 
	#[yaserde(rename = "FchTopeInf", prefix = "wsfev1", default)]
	pub fch_tope_inf: Option<String>, 
	#[yaserde(rename = "FchProceso", prefix = "wsfev1", default)]
	pub fch_proceso: Option<String>, 
	#[yaserde(rename = "Observaciones", prefix = "wsfev1", default)]
	pub observaciones: Option<ArrayOfObs>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoConsultar",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaeasinMovimientoConsultar {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "CAEA", prefix = "wsfev1", default)]
	pub caea: Option<String>, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoConsultarResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaeasinMovimientoConsultarResponse {
	#[yaserde(rename = "FECAEASinMovimientoConsultarResult", prefix = "wsfev1", default)]
	pub fecaea_sin_movimiento_consultar_result: Option<FecaeasinMovConsResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovConsResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeasinMovConsResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfFECAEASinMov>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFECAEASinMov",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfFECAEASinMov {
	#[yaserde(rename = "FECAEASinMov", prefix = "wsfev1", default)]
	pub fecaea_sin_mov: Vec<FecaeasinMov>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMov",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeasinMov {
	#[yaserde(rename = "CAEA", prefix = "wsfev1", default)]
	pub caea: Option<String>, 
	#[yaserde(rename = "FchProceso", prefix = "wsfev1", default)]
	pub fch_proceso: Option<String>, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoInformar",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaeasinMovimientoInformar {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "PtoVta", prefix = "wsfev1", default)]
	pub pto_vta: i32, 
	#[yaserde(rename = "CAEA", prefix = "wsfev1", default)]
	pub caea: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovimientoInformarResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaeasinMovimientoInformarResponse {
	#[yaserde(rename = "FECAEASinMovimientoInformarResult", prefix = "wsfev1", default)]
	pub fecaea_sin_movimiento_informar_result: Option<FecaeasinMovResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEASinMovResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecaeasinMovResponse {
	#[yaserde(flatten, default)]
	pub fecaeasin_mov: FecaeasinMov, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Resultado", prefix = "wsfev1", default)]
	pub resultado: Option<String>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAConsultar",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct Fecaeaconsultar {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "Periodo", prefix = "wsfev1", default)]
	pub periodo: i32, 
	#[yaserde(rename = "Orden", prefix = "wsfev1", default)]
	pub orden: i16, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECAEAConsultarResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FecaeaconsultarResponse {
	#[yaserde(rename = "FECAEAConsultarResult", prefix = "wsfev1", default)]
	pub fecaea_consultar_result: Option<FecaeagetResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetCotizacion",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetCotizacion {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
	#[yaserde(rename = "MonId", prefix = "wsfev1", default)]
	pub mon_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetCotizacionResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetCotizacionResponse {
	#[yaserde(rename = "FEParamGetCotizacionResult", prefix = "wsfev1", default)]
	pub fe_param_get_cotizacion_result: Option<FecotizacionResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FECotizacionResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FecotizacionResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<Cotizacion>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Cotizacion",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Cotizacion {
	#[yaserde(rename = "MonId", prefix = "wsfev1", default)]
	pub mon_id: Option<String>, 
	#[yaserde(rename = "MonCotiz", prefix = "wsfev1", default)]
	pub mon_cotiz: f64, 
	#[yaserde(rename = "FchCotiz", prefix = "wsfev1", default)]
	pub fch_cotiz: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposTributos",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposTributos {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposTributosResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposTributosResponse {
	#[yaserde(rename = "FEParamGetTiposTributosResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_tributos_result: Option<FetributoResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FETributoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FetributoResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfTributoTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTributoTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfTributoTipo {
	#[yaserde(rename = "TributoTipo", prefix = "wsfev1", default)]
	pub tributo_tipo: Vec<TributoTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TributoTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct TributoTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i16, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposMonedas",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposMonedas {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposMonedasResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposMonedasResponse {
	#[yaserde(rename = "FEParamGetTiposMonedasResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_monedas_result: Option<MonedaResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MonedaResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct MonedaResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfMoneda>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMoneda",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfMoneda {
	#[yaserde(rename = "Moneda", prefix = "wsfev1", default)]
	pub moneda: Vec<Moneda>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Moneda",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct Moneda {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposIva",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposIva {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposIvaResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposIvaResponse {
	#[yaserde(rename = "FEParamGetTiposIvaResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_iva_result: Option<IvaTipoResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IvaTipoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct IvaTipoResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfIvaTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfIvaTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfIvaTipo {
	#[yaserde(rename = "IvaTipo", prefix = "wsfev1", default)]
	pub iva_tipo: Vec<IvaTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IvaTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct IvaTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposOpcional",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposOpcional {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposOpcionalResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposOpcionalResponse {
	#[yaserde(rename = "FEParamGetTiposOpcionalResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_opcional_result: Option<OpcionalTipoResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OpcionalTipoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct OpcionalTipoResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfOpcionalTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfOpcionalTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfOpcionalTipo {
	#[yaserde(rename = "OpcionalTipo", prefix = "wsfev1", default)]
	pub opcional_tipo: Vec<OpcionalTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OpcionalTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct OpcionalTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposConcepto",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposConcepto {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposConceptoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposConceptoResponse {
	#[yaserde(rename = "FEParamGetTiposConceptoResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_concepto_result: Option<ConceptoTipoResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConceptoTipoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ConceptoTipoResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfConceptoTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfConceptoTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfConceptoTipo {
	#[yaserde(rename = "ConceptoTipo", prefix = "wsfev1", default)]
	pub concepto_tipo: Vec<ConceptoTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConceptoTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ConceptoTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i32, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetPtosVenta",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetPtosVenta {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetPtosVentaResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetPtosVentaResponse {
	#[yaserde(rename = "FEParamGetPtosVentaResult", prefix = "wsfev1", default)]
	pub fe_param_get_ptos_venta_result: Option<FeptoVentaResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEPtoVentaResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FeptoVentaResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfPtoVenta>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPtoVenta",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfPtoVenta {
	#[yaserde(rename = "PtoVenta", prefix = "wsfev1", default)]
	pub pto_venta: Vec<PtoVenta>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PtoVenta",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct PtoVenta {
	#[yaserde(rename = "Nro", prefix = "wsfev1", default)]
	pub nro: i32, 
	#[yaserde(rename = "EmisionTipo", prefix = "wsfev1", default)]
	pub emision_tipo: Option<String>, 
	#[yaserde(rename = "Bloqueado", prefix = "wsfev1", default)]
	pub bloqueado: Option<String>, 
	#[yaserde(rename = "FchBaja", prefix = "wsfev1", default)]
	pub fch_baja: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposCbte",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposCbte {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposCbteResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposCbteResponse {
	#[yaserde(rename = "FEParamGetTiposCbteResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_cbte_result: Option<CbteTipoResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CbteTipoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct CbteTipoResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfCbteTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCbteTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfCbteTipo {
	#[yaserde(rename = "CbteTipo", prefix = "wsfev1", default)]
	pub cbte_tipo: Vec<CbteTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CbteTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct CbteTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i32, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposDoc",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposDoc {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposDocResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposDocResponse {
	#[yaserde(rename = "FEParamGetTiposDocResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_doc_result: Option<DocTipoResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DocTipoResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct DocTipoResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfDocTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDocTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfDocTipo {
	#[yaserde(rename = "DocTipo", prefix = "wsfev1", default)]
	pub doc_tipo: Vec<DocTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DocTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct DocTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i32, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
	#[yaserde(rename = "FchDesde", prefix = "wsfev1", default)]
	pub fch_desde: Option<String>, 
	#[yaserde(rename = "FchHasta", prefix = "wsfev1", default)]
	pub fch_hasta: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposPaises",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposPaises {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetTiposPaisesResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetTiposPaisesResponse {
	#[yaserde(rename = "FEParamGetTiposPaisesResult", prefix = "wsfev1", default)]
	pub fe_param_get_tipos_paises_result: Option<FepaisResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEPaisResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FepaisResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfPaisTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPaisTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfPaisTipo {
	#[yaserde(rename = "PaisTipo", prefix = "wsfev1", default)]
	pub pais_tipo: Vec<PaisTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PaisTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct PaisTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i16, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetActividades",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetActividades {
	#[yaserde(rename = "Auth", prefix = "wsfev1", default)]
	pub auth: Option<FeauthRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEParamGetActividadesResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "wsfev1",
)]
pub struct FeparamGetActividadesResponse {
	#[yaserde(rename = "FEParamGetActividadesResult", prefix = "wsfev1", default)]
	pub fe_param_get_actividades_result: Option<FeactividadesResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FEActividadesResponse",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct FeactividadesResponse {
	#[yaserde(rename = "ResultGet", prefix = "wsfev1", default)]
	pub result_get: Option<ArrayOfActividadesTipo>, 
	#[yaserde(rename = "Errors", prefix = "wsfev1", default)]
	pub errors: Option<ArrayOfErr>, 
	#[yaserde(rename = "Events", prefix = "wsfev1", default)]
	pub events: Option<ArrayOfEvt>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfActividadesTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ArrayOfActividadesTipo {
	#[yaserde(rename = "ActividadesTipo", prefix = "wsfev1", default)]
	pub actividades_tipo: Vec<ActividadesTipo>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ActividadesTipo",
	namespace = "wsfev1: http://ar.gov.afip.dif.FEV1/",
	prefix = "wsfev1",
)]
pub struct ActividadesTipo {
	#[yaserde(rename = "Id", prefix = "wsfev1", default)]
	pub id: i64, 
	#[yaserde(rename = "Orden", prefix = "wsfev1", default)]
	pub orden: i16, 
	#[yaserde(rename = "Desc", prefix = "wsfev1", default)]
	pub desc: Option<String>, 
}
}

pub mod ports {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            pub type FecaesolicitarSoapIn = messages::FecaesolicitarSoapIn;

pub type FecaesolicitarSoapOut = messages::FecaesolicitarSoapOut;

pub type FecompTotXRequestSoapIn = messages::FecompTotXRequestSoapIn;

pub type FecompTotXRequestSoapOut = messages::FecompTotXRequestSoapOut;

pub type FedummySoapIn = messages::FedummySoapIn;

pub type FedummySoapOut = messages::FedummySoapOut;

pub type FecompUltimoAutorizadoSoapIn = messages::FecompUltimoAutorizadoSoapIn;

pub type FecompUltimoAutorizadoSoapOut = messages::FecompUltimoAutorizadoSoapOut;

pub type FecompConsultarSoapIn = messages::FecompConsultarSoapIn;

pub type FecompConsultarSoapOut = messages::FecompConsultarSoapOut;

pub type FecaearegInformativoSoapIn = messages::FecaearegInformativoSoapIn;

pub type FecaearegInformativoSoapOut = messages::FecaearegInformativoSoapOut;

pub type FecaeasolicitarSoapIn = messages::FecaeasolicitarSoapIn;

pub type FecaeasolicitarSoapOut = messages::FecaeasolicitarSoapOut;

pub type FecaeasinMovimientoConsultarSoapIn = messages::FecaeasinMovimientoConsultarSoapIn;

pub type FecaeasinMovimientoConsultarSoapOut = messages::FecaeasinMovimientoConsultarSoapOut;

pub type FecaeasinMovimientoInformarSoapIn = messages::FecaeasinMovimientoInformarSoapIn;

pub type FecaeasinMovimientoInformarSoapOut = messages::FecaeasinMovimientoInformarSoapOut;

pub type FecaeaconsultarSoapIn = messages::FecaeaconsultarSoapIn;

pub type FecaeaconsultarSoapOut = messages::FecaeaconsultarSoapOut;

pub type FeparamGetCotizacionSoapIn = messages::FeparamGetCotizacionSoapIn;

pub type FeparamGetCotizacionSoapOut = messages::FeparamGetCotizacionSoapOut;

pub type FeparamGetTiposTributosSoapIn = messages::FeparamGetTiposTributosSoapIn;

pub type FeparamGetTiposTributosSoapOut = messages::FeparamGetTiposTributosSoapOut;

pub type FeparamGetTiposMonedasSoapIn = messages::FeparamGetTiposMonedasSoapIn;

pub type FeparamGetTiposMonedasSoapOut = messages::FeparamGetTiposMonedasSoapOut;

pub type FeparamGetTiposIvaSoapIn = messages::FeparamGetTiposIvaSoapIn;

pub type FeparamGetTiposIvaSoapOut = messages::FeparamGetTiposIvaSoapOut;

pub type FeparamGetTiposOpcionalSoapIn = messages::FeparamGetTiposOpcionalSoapIn;

pub type FeparamGetTiposOpcionalSoapOut = messages::FeparamGetTiposOpcionalSoapOut;

pub type FeparamGetTiposConceptoSoapIn = messages::FeparamGetTiposConceptoSoapIn;

pub type FeparamGetTiposConceptoSoapOut = messages::FeparamGetTiposConceptoSoapOut;

pub type FeparamGetPtosVentaSoapIn = messages::FeparamGetPtosVentaSoapIn;

pub type FeparamGetPtosVentaSoapOut = messages::FeparamGetPtosVentaSoapOut;

pub type FeparamGetTiposCbteSoapIn = messages::FeparamGetTiposCbteSoapIn;

pub type FeparamGetTiposCbteSoapOut = messages::FeparamGetTiposCbteSoapOut;

pub type FeparamGetTiposDocSoapIn = messages::FeparamGetTiposDocSoapIn;

pub type FeparamGetTiposDocSoapOut = messages::FeparamGetTiposDocSoapOut;

pub type FeparamGetTiposPaisesSoapIn = messages::FeparamGetTiposPaisesSoapIn;

pub type FeparamGetTiposPaisesSoapOut = messages::FeparamGetTiposPaisesSoapOut;

pub type FeparamGetActividadesSoapIn = messages::FeparamGetActividadesSoapIn;

pub type FeparamGetActividadesSoapOut = messages::FeparamGetActividadesSoapOut;

#[async_trait]
pub trait ServiceSoap {
	async fn fecae_solicitar (&self, fecaesolicitar_soap_in: FecaesolicitarSoapIn) -> Result<FecaesolicitarSoapOut,Option<SoapFault>>;
	async fn fe_comp_tot_x_request (&self, fecomp_tot_x_request_soap_in: FecompTotXRequestSoapIn) -> Result<FecompTotXRequestSoapOut,Option<SoapFault>>;
	async fn fe_dummy (&self, fedummy_soap_in: FedummySoapIn) -> Result<FedummySoapOut,Option<SoapFault>>;
	async fn fe_comp_ultimo_autorizado (&self, fecomp_ultimo_autorizado_soap_in: FecompUltimoAutorizadoSoapIn) -> Result<FecompUltimoAutorizadoSoapOut,Option<SoapFault>>;
	async fn fe_comp_consultar (&self, fecomp_consultar_soap_in: FecompConsultarSoapIn) -> Result<FecompConsultarSoapOut,Option<SoapFault>>;
	async fn fecaea_reg_informativo (&self, fecaeareg_informativo_soap_in: FecaearegInformativoSoapIn) -> Result<FecaearegInformativoSoapOut,Option<SoapFault>>;
	async fn fecaea_solicitar (&self, fecaeasolicitar_soap_in: FecaeasolicitarSoapIn) -> Result<FecaeasolicitarSoapOut,Option<SoapFault>>;
	async fn fecaea_sin_movimiento_consultar (&self, fecaeasin_movimiento_consultar_soap_in: FecaeasinMovimientoConsultarSoapIn) -> Result<FecaeasinMovimientoConsultarSoapOut,Option<SoapFault>>;
	async fn fecaea_sin_movimiento_informar (&self, fecaeasin_movimiento_informar_soap_in: FecaeasinMovimientoInformarSoapIn) -> Result<FecaeasinMovimientoInformarSoapOut,Option<SoapFault>>;
	async fn fecaea_consultar (&self, fecaeaconsultar_soap_in: FecaeaconsultarSoapIn) -> Result<FecaeaconsultarSoapOut,Option<SoapFault>>;
	async fn fe_param_get_cotizacion (&self, feparam_get_cotizacion_soap_in: FeparamGetCotizacionSoapIn) -> Result<FeparamGetCotizacionSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_tributos (&self, feparam_get_tipos_tributos_soap_in: FeparamGetTiposTributosSoapIn) -> Result<FeparamGetTiposTributosSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_monedas (&self, feparam_get_tipos_monedas_soap_in: FeparamGetTiposMonedasSoapIn) -> Result<FeparamGetTiposMonedasSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_iva (&self, feparam_get_tipos_iva_soap_in: FeparamGetTiposIvaSoapIn) -> Result<FeparamGetTiposIvaSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_opcional (&self, feparam_get_tipos_opcional_soap_in: FeparamGetTiposOpcionalSoapIn) -> Result<FeparamGetTiposOpcionalSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_concepto (&self, feparam_get_tipos_concepto_soap_in: FeparamGetTiposConceptoSoapIn) -> Result<FeparamGetTiposConceptoSoapOut,Option<SoapFault>>;
	async fn fe_param_get_ptos_venta (&self, feparam_get_ptos_venta_soap_in: FeparamGetPtosVentaSoapIn) -> Result<FeparamGetPtosVentaSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_cbte (&self, feparam_get_tipos_cbte_soap_in: FeparamGetTiposCbteSoapIn) -> Result<FeparamGetTiposCbteSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_doc (&self, feparam_get_tipos_doc_soap_in: FeparamGetTiposDocSoapIn) -> Result<FeparamGetTiposDocSoapOut,Option<SoapFault>>;
	async fn fe_param_get_tipos_paises (&self, feparam_get_tipos_paises_soap_in: FeparamGetTiposPaisesSoapIn) -> Result<FeparamGetTiposPaisesSoapOut,Option<SoapFault>>;
	async fn fe_param_get_actividades (&self, feparam_get_actividades_soap_in: FeparamGetActividadesSoapIn) -> Result<FeparamGetActividadesSoapOut,Option<SoapFault>>;
}
}

pub mod bindings {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            
            impl ServiceSoap {
                async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
                    let body = to_string(request).expect("failed to generate xml");
                    debug!("SOAP Request: {}", body);
                    let mut req = self
                        .client
                        .post(&self.url)
                        .body(body)
                        .header("Content-Type", "text/xml")
                        .header("Soapaction", action);
                    if let Some(credentials) = &self.credentials {
                        req = req.basic_auth(
                            credentials.0.to_string(),
                            Option::Some(credentials.1.to_string()),
                        );
                    }
                    let res = req.send().await?;
                    let status = res.status();
                    debug!("SOAP Status: {}", status);
                    let txt = res.text().await.unwrap_or_default();
                    debug!("SOAP Response: {}", txt);
                    Ok((status, txt))
                }
            }
            #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaesolicitarSoapIn {
                        #[yaserde(rename = "FECAESolicitar", default)]
                        pub body: ports::FecaesolicitarSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaesolicitarSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaesolicitarSoapIn,
        }
        
        impl FecaesolicitarSoapInSoapEnvelope {
            pub fn new(body: SoapFecaesolicitarSoapIn) -> Self {
                FecaesolicitarSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaesolicitarSoapOut {
                    #[yaserde(rename = "FecaesolicitarSoapOut", default)]
                    pub body: ports::FecaesolicitarSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaesolicitarSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaesolicitarSoapOut,
        }
        
        impl FecaesolicitarSoapOutSoapEnvelope {
            pub fn new(body: SoapFecaesolicitarSoapOut) -> Self {
                FecaesolicitarSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecompTotXRequestSoapIn {
                        #[yaserde(rename = "FECompTotXRequest", default)]
                        pub body: ports::FecompTotXRequestSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecompTotXRequestSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecompTotXRequestSoapIn,
        }
        
        impl FecompTotXRequestSoapInSoapEnvelope {
            pub fn new(body: SoapFecompTotXRequestSoapIn) -> Self {
                FecompTotXRequestSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecompTotXRequestSoapOut {
                    #[yaserde(rename = "FecompTotXRequestSoapOut", default)]
                    pub body: ports::FecompTotXRequestSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecompTotXRequestSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecompTotXRequestSoapOut,
        }
        
        impl FecompTotXRequestSoapOutSoapEnvelope {
            pub fn new(body: SoapFecompTotXRequestSoapOut) -> Self {
                FecompTotXRequestSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFedummySoapIn {
                        #[yaserde(rename = "FEDummy", default)]
                        pub body: ports::FedummySoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FedummySoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFedummySoapIn,
        }
        
        impl FedummySoapInSoapEnvelope {
            pub fn new(body: SoapFedummySoapIn) -> Self {
                FedummySoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFedummySoapOut {
                    #[yaserde(rename = "FedummySoapOut", default)]
                    pub body: ports::FedummySoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FedummySoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFedummySoapOut,
        }
        
        impl FedummySoapOutSoapEnvelope {
            pub fn new(body: SoapFedummySoapOut) -> Self {
                FedummySoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecompUltimoAutorizadoSoapIn {
                        #[yaserde(rename = "FECompUltimoAutorizado", default)]
                        pub body: ports::FecompUltimoAutorizadoSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecompUltimoAutorizadoSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecompUltimoAutorizadoSoapIn,
        }
        
        impl FecompUltimoAutorizadoSoapInSoapEnvelope {
            pub fn new(body: SoapFecompUltimoAutorizadoSoapIn) -> Self {
                FecompUltimoAutorizadoSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecompUltimoAutorizadoSoapOut {
                    #[yaserde(rename = "FecompUltimoAutorizadoSoapOut", default)]
                    pub body: ports::FecompUltimoAutorizadoSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecompUltimoAutorizadoSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecompUltimoAutorizadoSoapOut,
        }
        
        impl FecompUltimoAutorizadoSoapOutSoapEnvelope {
            pub fn new(body: SoapFecompUltimoAutorizadoSoapOut) -> Self {
                FecompUltimoAutorizadoSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecompConsultarSoapIn {
                        #[yaserde(rename = "FECompConsultar", default)]
                        pub body: ports::FecompConsultarSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecompConsultarSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecompConsultarSoapIn,
        }
        
        impl FecompConsultarSoapInSoapEnvelope {
            pub fn new(body: SoapFecompConsultarSoapIn) -> Self {
                FecompConsultarSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecompConsultarSoapOut {
                    #[yaserde(rename = "FecompConsultarSoapOut", default)]
                    pub body: ports::FecompConsultarSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecompConsultarSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecompConsultarSoapOut,
        }
        
        impl FecompConsultarSoapOutSoapEnvelope {
            pub fn new(body: SoapFecompConsultarSoapOut) -> Self {
                FecompConsultarSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaearegInformativoSoapIn {
                        #[yaserde(rename = "FECAEARegInformativo", default)]
                        pub body: ports::FecaearegInformativoSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaearegInformativoSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaearegInformativoSoapIn,
        }
        
        impl FecaearegInformativoSoapInSoapEnvelope {
            pub fn new(body: SoapFecaearegInformativoSoapIn) -> Self {
                FecaearegInformativoSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaearegInformativoSoapOut {
                    #[yaserde(rename = "FecaearegInformativoSoapOut", default)]
                    pub body: ports::FecaearegInformativoSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaearegInformativoSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaearegInformativoSoapOut,
        }
        
        impl FecaearegInformativoSoapOutSoapEnvelope {
            pub fn new(body: SoapFecaearegInformativoSoapOut) -> Self {
                FecaearegInformativoSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeasolicitarSoapIn {
                        #[yaserde(rename = "FECAEASolicitar", default)]
                        pub body: ports::FecaeasolicitarSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeasolicitarSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeasolicitarSoapIn,
        }
        
        impl FecaeasolicitarSoapInSoapEnvelope {
            pub fn new(body: SoapFecaeasolicitarSoapIn) -> Self {
                FecaeasolicitarSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeasolicitarSoapOut {
                    #[yaserde(rename = "FecaeasolicitarSoapOut", default)]
                    pub body: ports::FecaeasolicitarSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeasolicitarSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeasolicitarSoapOut,
        }
        
        impl FecaeasolicitarSoapOutSoapEnvelope {
            pub fn new(body: SoapFecaeasolicitarSoapOut) -> Self {
                FecaeasolicitarSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeasinMovimientoConsultarSoapIn {
                        #[yaserde(rename = "FECAEASinMovimientoConsultar", default)]
                        pub body: ports::FecaeasinMovimientoConsultarSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeasinMovimientoConsultarSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeasinMovimientoConsultarSoapIn,
        }
        
        impl FecaeasinMovimientoConsultarSoapInSoapEnvelope {
            pub fn new(body: SoapFecaeasinMovimientoConsultarSoapIn) -> Self {
                FecaeasinMovimientoConsultarSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeasinMovimientoConsultarSoapOut {
                    #[yaserde(rename = "FecaeasinMovimientoConsultarSoapOut", default)]
                    pub body: ports::FecaeasinMovimientoConsultarSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeasinMovimientoConsultarSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeasinMovimientoConsultarSoapOut,
        }
        
        impl FecaeasinMovimientoConsultarSoapOutSoapEnvelope {
            pub fn new(body: SoapFecaeasinMovimientoConsultarSoapOut) -> Self {
                FecaeasinMovimientoConsultarSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeasinMovimientoInformarSoapIn {
                        #[yaserde(rename = "FECAEASinMovimientoInformar", default)]
                        pub body: ports::FecaeasinMovimientoInformarSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeasinMovimientoInformarSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeasinMovimientoInformarSoapIn,
        }
        
        impl FecaeasinMovimientoInformarSoapInSoapEnvelope {
            pub fn new(body: SoapFecaeasinMovimientoInformarSoapIn) -> Self {
                FecaeasinMovimientoInformarSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeasinMovimientoInformarSoapOut {
                    #[yaserde(rename = "FecaeasinMovimientoInformarSoapOut", default)]
                    pub body: ports::FecaeasinMovimientoInformarSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeasinMovimientoInformarSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeasinMovimientoInformarSoapOut,
        }
        
        impl FecaeasinMovimientoInformarSoapOutSoapEnvelope {
            pub fn new(body: SoapFecaeasinMovimientoInformarSoapOut) -> Self {
                FecaeasinMovimientoInformarSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeaconsultarSoapIn {
                        #[yaserde(rename = "FECAEAConsultar", default)]
                        pub body: ports::FecaeaconsultarSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeaconsultarSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeaconsultarSoapIn,
        }
        
        impl FecaeaconsultarSoapInSoapEnvelope {
            pub fn new(body: SoapFecaeaconsultarSoapIn) -> Self {
                FecaeaconsultarSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFecaeaconsultarSoapOut {
                    #[yaserde(rename = "FecaeaconsultarSoapOut", default)]
                    pub body: ports::FecaeaconsultarSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FecaeaconsultarSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFecaeaconsultarSoapOut,
        }
        
        impl FecaeaconsultarSoapOutSoapEnvelope {
            pub fn new(body: SoapFecaeaconsultarSoapOut) -> Self {
                FecaeaconsultarSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetCotizacionSoapIn {
                        #[yaserde(rename = "FEParamGetCotizacion", default)]
                        pub body: ports::FeparamGetCotizacionSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetCotizacionSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetCotizacionSoapIn,
        }
        
        impl FeparamGetCotizacionSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetCotizacionSoapIn) -> Self {
                FeparamGetCotizacionSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetCotizacionSoapOut {
                    #[yaserde(rename = "FeparamGetCotizacionSoapOut", default)]
                    pub body: ports::FeparamGetCotizacionSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetCotizacionSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetCotizacionSoapOut,
        }
        
        impl FeparamGetCotizacionSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetCotizacionSoapOut) -> Self {
                FeparamGetCotizacionSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposTributosSoapIn {
                        #[yaserde(rename = "FEParamGetTiposTributos", default)]
                        pub body: ports::FeparamGetTiposTributosSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposTributosSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposTributosSoapIn,
        }
        
        impl FeparamGetTiposTributosSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposTributosSoapIn) -> Self {
                FeparamGetTiposTributosSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposTributosSoapOut {
                    #[yaserde(rename = "FeparamGetTiposTributosSoapOut", default)]
                    pub body: ports::FeparamGetTiposTributosSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposTributosSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposTributosSoapOut,
        }
        
        impl FeparamGetTiposTributosSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposTributosSoapOut) -> Self {
                FeparamGetTiposTributosSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposMonedasSoapIn {
                        #[yaserde(rename = "FEParamGetTiposMonedas", default)]
                        pub body: ports::FeparamGetTiposMonedasSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposMonedasSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposMonedasSoapIn,
        }
        
        impl FeparamGetTiposMonedasSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposMonedasSoapIn) -> Self {
                FeparamGetTiposMonedasSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposMonedasSoapOut {
                    #[yaserde(rename = "FeparamGetTiposMonedasSoapOut", default)]
                    pub body: ports::FeparamGetTiposMonedasSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposMonedasSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposMonedasSoapOut,
        }
        
        impl FeparamGetTiposMonedasSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposMonedasSoapOut) -> Self {
                FeparamGetTiposMonedasSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposIvaSoapIn {
                        #[yaserde(rename = "FEParamGetTiposIva", default)]
                        pub body: ports::FeparamGetTiposIvaSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposIvaSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposIvaSoapIn,
        }
        
        impl FeparamGetTiposIvaSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposIvaSoapIn) -> Self {
                FeparamGetTiposIvaSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposIvaSoapOut {
                    #[yaserde(rename = "FeparamGetTiposIvaSoapOut", default)]
                    pub body: ports::FeparamGetTiposIvaSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposIvaSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposIvaSoapOut,
        }
        
        impl FeparamGetTiposIvaSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposIvaSoapOut) -> Self {
                FeparamGetTiposIvaSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposOpcionalSoapIn {
                        #[yaserde(rename = "FEParamGetTiposOpcional", default)]
                        pub body: ports::FeparamGetTiposOpcionalSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposOpcionalSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposOpcionalSoapIn,
        }
        
        impl FeparamGetTiposOpcionalSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposOpcionalSoapIn) -> Self {
                FeparamGetTiposOpcionalSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposOpcionalSoapOut {
                    #[yaserde(rename = "FeparamGetTiposOpcionalSoapOut", default)]
                    pub body: ports::FeparamGetTiposOpcionalSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposOpcionalSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposOpcionalSoapOut,
        }
        
        impl FeparamGetTiposOpcionalSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposOpcionalSoapOut) -> Self {
                FeparamGetTiposOpcionalSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposConceptoSoapIn {
                        #[yaserde(rename = "FEParamGetTiposConcepto", default)]
                        pub body: ports::FeparamGetTiposConceptoSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposConceptoSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposConceptoSoapIn,
        }
        
        impl FeparamGetTiposConceptoSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposConceptoSoapIn) -> Self {
                FeparamGetTiposConceptoSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposConceptoSoapOut {
                    #[yaserde(rename = "FeparamGetTiposConceptoSoapOut", default)]
                    pub body: ports::FeparamGetTiposConceptoSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposConceptoSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposConceptoSoapOut,
        }
        
        impl FeparamGetTiposConceptoSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposConceptoSoapOut) -> Self {
                FeparamGetTiposConceptoSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetPtosVentaSoapIn {
                        #[yaserde(rename = "FEParamGetPtosVenta", default)]
                        pub body: ports::FeparamGetPtosVentaSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetPtosVentaSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetPtosVentaSoapIn,
        }
        
        impl FeparamGetPtosVentaSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetPtosVentaSoapIn) -> Self {
                FeparamGetPtosVentaSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetPtosVentaSoapOut {
                    #[yaserde(rename = "FeparamGetPtosVentaSoapOut", default)]
                    pub body: ports::FeparamGetPtosVentaSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetPtosVentaSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetPtosVentaSoapOut,
        }
        
        impl FeparamGetPtosVentaSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetPtosVentaSoapOut) -> Self {
                FeparamGetPtosVentaSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposCbteSoapIn {
                        #[yaserde(rename = "FEParamGetTiposCbte", default)]
                        pub body: ports::FeparamGetTiposCbteSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposCbteSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposCbteSoapIn,
        }
        
        impl FeparamGetTiposCbteSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposCbteSoapIn) -> Self {
                FeparamGetTiposCbteSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposCbteSoapOut {
                    #[yaserde(rename = "FeparamGetTiposCbteSoapOut", default)]
                    pub body: ports::FeparamGetTiposCbteSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposCbteSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposCbteSoapOut,
        }
        
        impl FeparamGetTiposCbteSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposCbteSoapOut) -> Self {
                FeparamGetTiposCbteSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposDocSoapIn {
                        #[yaserde(rename = "FEParamGetTiposDoc", default)]
                        pub body: ports::FeparamGetTiposDocSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposDocSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposDocSoapIn,
        }
        
        impl FeparamGetTiposDocSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposDocSoapIn) -> Self {
                FeparamGetTiposDocSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposDocSoapOut {
                    #[yaserde(rename = "FeparamGetTiposDocSoapOut", default)]
                    pub body: ports::FeparamGetTiposDocSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposDocSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposDocSoapOut,
        }
        
        impl FeparamGetTiposDocSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposDocSoapOut) -> Self {
                FeparamGetTiposDocSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposPaisesSoapIn {
                        #[yaserde(rename = "FEParamGetTiposPaises", default)]
                        pub body: ports::FeparamGetTiposPaisesSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposPaisesSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposPaisesSoapIn,
        }
        
        impl FeparamGetTiposPaisesSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposPaisesSoapIn) -> Self {
                FeparamGetTiposPaisesSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetTiposPaisesSoapOut {
                    #[yaserde(rename = "FeparamGetTiposPaisesSoapOut", default)]
                    pub body: ports::FeparamGetTiposPaisesSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetTiposPaisesSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetTiposPaisesSoapOut,
        }
        
        impl FeparamGetTiposPaisesSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetTiposPaisesSoapOut) -> Self {
                FeparamGetTiposPaisesSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetActividadesSoapIn {
                        #[yaserde(rename = "FEParamGetActividades", default)]
                        pub body: ports::FeparamGetActividadesSoapIn,
                        #[yaserde(attribute)]
                        pub xmlns: Option<String>,
                    }
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetActividadesSoapInSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetActividadesSoapIn,
        }
        
        impl FeparamGetActividadesSoapInSoapEnvelope {
            pub fn new(body: SoapFeparamGetActividadesSoapIn) -> Self {
                FeparamGetActividadesSoapInSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
                    pub struct SoapFeparamGetActividadesSoapOut {
                    #[yaserde(rename = "FeparamGetActividadesSoapOut", default)]
                    pub body: ports::FeparamGetActividadesSoapOut,
                         #[yaserde(rename = "Fault", default)]
                            pub fault: Option<SoapFault>,
                            
                }
                #[derive(Debug, Default, YaSerialize, YaDeserialize)]
        #[yaserde(
            rename = "Envelope",
            namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
            prefix = "soapenv"
        )]
        pub struct FeparamGetActividadesSoapOutSoapEnvelope {
            #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
            pub encoding_style: String,
            #[yaserde(rename = "wsfev1", prefix = "xmlns", attribute)]
            pub tnsattr: Option<String>,
            #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
            pub urnattr: Option<String>,
            #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
            pub xsiattr: Option<String>,
            #[yaserde(rename = "Header", prefix = "soapenv")]
            pub header: Option<Header>,
            #[yaserde(rename = "Body", prefix = "soapenv")]
            pub body: SoapFeparamGetActividadesSoapOut,
        }
        
        impl FeparamGetActividadesSoapOutSoapEnvelope {
            pub fn new(body: SoapFeparamGetActividadesSoapOut) -> Self {
                FeparamGetActividadesSoapOutSoapEnvelope {
                    encoding_style: SOAP_ENCODING.to_string(),
                    tnsattr: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
                    body,
                    urnattr: None,
                    xsiattr: None,
                    header: None,
                }
            }
        }        
        
                impl Default for ServiceSoap {
                fn default() -> Self {
                    ServiceSoap {
                        client: reqwest::Client::new(),
                        url: "http://ar.gov.afip.dif.FEV1/".to_string(),
                        credentials: Option::None,
                     }
                }
            }
            impl ServiceSoap {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
                    ServiceSoap {
                        client: reqwest::Client::new(),
                        url: url.to_string(),
                        credentials,
                    }
                }
        }
        pub struct ServiceSoap {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                #[async_trait]
	impl ports::ServiceSoap for ServiceSoap {
	async fn fecae_solicitar (&self, fecaesolicitar_soap_in: ports::FecaesolicitarSoapIn) -> Result<ports::FecaesolicitarSoapOut, Option<SoapFault>> {

        let __request = FecaesolicitarSoapInSoapEnvelope::new(SoapFecaesolicitarSoapIn {
            body: fecaesolicitar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAESolicitar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaesolicitarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_comp_tot_x_request (&self, fecomp_tot_x_request_soap_in: ports::FecompTotXRequestSoapIn) -> Result<ports::FecompTotXRequestSoapOut, Option<SoapFault>> {

        let __request = FecompTotXRequestSoapInSoapEnvelope::new(SoapFecompTotXRequestSoapIn {
            body: fecomp_tot_x_request_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECompTotXRequest")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecompTotXRequestSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_dummy (&self, fedummy_soap_in: ports::FedummySoapIn) -> Result<ports::FedummySoapOut, Option<SoapFault>> {

        let __request = FedummySoapInSoapEnvelope::new(SoapFedummySoapIn {
            body: fedummy_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEDummy")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FedummySoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_comp_ultimo_autorizado (&self, fecomp_ultimo_autorizado_soap_in: ports::FecompUltimoAutorizadoSoapIn) -> Result<ports::FecompUltimoAutorizadoSoapOut, Option<SoapFault>> {

        let __request = FecompUltimoAutorizadoSoapInSoapEnvelope::new(SoapFecompUltimoAutorizadoSoapIn {
            body: fecomp_ultimo_autorizado_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECompUltimoAutorizado")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecompUltimoAutorizadoSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_comp_consultar (&self, fecomp_consultar_soap_in: ports::FecompConsultarSoapIn) -> Result<ports::FecompConsultarSoapOut, Option<SoapFault>> {

        let __request = FecompConsultarSoapInSoapEnvelope::new(SoapFecompConsultarSoapIn {
            body: fecomp_consultar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECompConsultar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecompConsultarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_reg_informativo (&self, fecaeareg_informativo_soap_in: ports::FecaearegInformativoSoapIn) -> Result<ports::FecaearegInformativoSoapOut, Option<SoapFault>> {

        let __request = FecaearegInformativoSoapInSoapEnvelope::new(SoapFecaearegInformativoSoapIn {
            body: fecaeareg_informativo_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEARegInformativo")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaearegInformativoSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_solicitar (&self, fecaeasolicitar_soap_in: ports::FecaeasolicitarSoapIn) -> Result<ports::FecaeasolicitarSoapOut, Option<SoapFault>> {

        let __request = FecaeasolicitarSoapInSoapEnvelope::new(SoapFecaeasolicitarSoapIn {
            body: fecaeasolicitar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEASolicitar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeasolicitarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_sin_movimiento_consultar (&self, fecaeasin_movimiento_consultar_soap_in: ports::FecaeasinMovimientoConsultarSoapIn) -> Result<ports::FecaeasinMovimientoConsultarSoapOut, Option<SoapFault>> {

        let __request = FecaeasinMovimientoConsultarSoapInSoapEnvelope::new(SoapFecaeasinMovimientoConsultarSoapIn {
            body: fecaeasin_movimiento_consultar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEASinMovimientoConsultar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeasinMovimientoConsultarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_sin_movimiento_informar (&self, fecaeasin_movimiento_informar_soap_in: ports::FecaeasinMovimientoInformarSoapIn) -> Result<ports::FecaeasinMovimientoInformarSoapOut, Option<SoapFault>> {

        let __request = FecaeasinMovimientoInformarSoapInSoapEnvelope::new(SoapFecaeasinMovimientoInformarSoapIn {
            body: fecaeasin_movimiento_informar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEASinMovimientoInformar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeasinMovimientoInformarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_consultar (&self, fecaeaconsultar_soap_in: ports::FecaeaconsultarSoapIn) -> Result<ports::FecaeaconsultarSoapOut, Option<SoapFault>> {

        let __request = FecaeaconsultarSoapInSoapEnvelope::new(SoapFecaeaconsultarSoapIn {
            body: fecaeaconsultar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEAConsultar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeaconsultarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_cotizacion (&self, feparam_get_cotizacion_soap_in: ports::FeparamGetCotizacionSoapIn) -> Result<ports::FeparamGetCotizacionSoapOut, Option<SoapFault>> {

        let __request = FeparamGetCotizacionSoapInSoapEnvelope::new(SoapFeparamGetCotizacionSoapIn {
            body: feparam_get_cotizacion_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetCotizacion")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetCotizacionSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_tributos (&self, feparam_get_tipos_tributos_soap_in: ports::FeparamGetTiposTributosSoapIn) -> Result<ports::FeparamGetTiposTributosSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposTributosSoapInSoapEnvelope::new(SoapFeparamGetTiposTributosSoapIn {
            body: feparam_get_tipos_tributos_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposTributos")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposTributosSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_monedas (&self, feparam_get_tipos_monedas_soap_in: ports::FeparamGetTiposMonedasSoapIn) -> Result<ports::FeparamGetTiposMonedasSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposMonedasSoapInSoapEnvelope::new(SoapFeparamGetTiposMonedasSoapIn {
            body: feparam_get_tipos_monedas_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposMonedas")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposMonedasSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_iva (&self, feparam_get_tipos_iva_soap_in: ports::FeparamGetTiposIvaSoapIn) -> Result<ports::FeparamGetTiposIvaSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposIvaSoapInSoapEnvelope::new(SoapFeparamGetTiposIvaSoapIn {
            body: feparam_get_tipos_iva_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposIva")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposIvaSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_opcional (&self, feparam_get_tipos_opcional_soap_in: ports::FeparamGetTiposOpcionalSoapIn) -> Result<ports::FeparamGetTiposOpcionalSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposOpcionalSoapInSoapEnvelope::new(SoapFeparamGetTiposOpcionalSoapIn {
            body: feparam_get_tipos_opcional_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposOpcional")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposOpcionalSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_concepto (&self, feparam_get_tipos_concepto_soap_in: ports::FeparamGetTiposConceptoSoapIn) -> Result<ports::FeparamGetTiposConceptoSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposConceptoSoapInSoapEnvelope::new(SoapFeparamGetTiposConceptoSoapIn {
            body: feparam_get_tipos_concepto_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposConcepto")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposConceptoSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_ptos_venta (&self, feparam_get_ptos_venta_soap_in: ports::FeparamGetPtosVentaSoapIn) -> Result<ports::FeparamGetPtosVentaSoapOut, Option<SoapFault>> {

        let __request = FeparamGetPtosVentaSoapInSoapEnvelope::new(SoapFeparamGetPtosVentaSoapIn {
            body: feparam_get_ptos_venta_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetPtosVenta")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetPtosVentaSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_cbte (&self, feparam_get_tipos_cbte_soap_in: ports::FeparamGetTiposCbteSoapIn) -> Result<ports::FeparamGetTiposCbteSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposCbteSoapInSoapEnvelope::new(SoapFeparamGetTiposCbteSoapIn {
            body: feparam_get_tipos_cbte_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposCbte")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposCbteSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_doc (&self, feparam_get_tipos_doc_soap_in: ports::FeparamGetTiposDocSoapIn) -> Result<ports::FeparamGetTiposDocSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposDocSoapInSoapEnvelope::new(SoapFeparamGetTiposDocSoapIn {
            body: feparam_get_tipos_doc_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposDoc")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposDocSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_paises (&self, feparam_get_tipos_paises_soap_in: ports::FeparamGetTiposPaisesSoapIn) -> Result<ports::FeparamGetTiposPaisesSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposPaisesSoapInSoapEnvelope::new(SoapFeparamGetTiposPaisesSoapIn {
            body: feparam_get_tipos_paises_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposPaises")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposPaisesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_actividades (&self, feparam_get_actividades_soap_in: ports::FeparamGetActividadesSoapIn) -> Result<ports::FeparamGetActividadesSoapOut, Option<SoapFault>> {

        let __request = FeparamGetActividadesSoapInSoapEnvelope::new(SoapFeparamGetActividadesSoapIn {
            body: feparam_get_actividades_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetActividades")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetActividadesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}}

            impl ServiceSoap12 {
                async fn send_soap_request<T: YaSerialize>(&self, request: &T, action: &str) -> SoapResponse {
                    let body = to_string(request).expect("failed to generate xml");
                    debug!("SOAP Request: {}", body);
                    let mut req = self
                        .client
                        .post(&self.url)
                        .body(body)
                        .header("Content-Type", "text/xml")
                        .header("Soapaction", action);
                    if let Some(credentials) = &self.credentials {
                        req = req.basic_auth(
                            credentials.0.to_string(),
                            Option::Some(credentials.1.to_string()),
                        );
                    }
                    let res = req.send().await?;
                    let status = res.status();
                    debug!("SOAP Status: {}", status);
                    let txt = res.text().await.unwrap_or_default();
                    debug!("SOAP Response: {}", txt);
                    Ok((status, txt))
                }
            }
            impl Default for ServiceSoap12 {
                fn default() -> Self {
                    ServiceSoap12 {
                        client: reqwest::Client::new(),
                        url: "http://ar.gov.afip.dif.FEV1/".to_string(),
                        credentials: Option::None,
                     }
                }
            }
            impl ServiceSoap12 {
                pub fn new(url: &str, credentials: Option<(String,String)>) -> Self {
                    ServiceSoap12 {
                        client: reqwest::Client::new(),
                        url: url.to_string(),
                        credentials,
                    }
                }
        }
        pub struct ServiceSoap12 {
                client: reqwest::Client,
                url: String,
                credentials: Option<(String,String)>
                }
                #[async_trait]
	impl ports::ServiceSoap for ServiceSoap12 {
	async fn fecae_solicitar (&self, fecaesolicitar_soap_in: ports::FecaesolicitarSoapIn) -> Result<ports::FecaesolicitarSoapOut, Option<SoapFault>> {

        let __request = FecaesolicitarSoapInSoapEnvelope::new(SoapFecaesolicitarSoapIn {
            body: fecaesolicitar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAESolicitar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaesolicitarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_comp_tot_x_request (&self, fecomp_tot_x_request_soap_in: ports::FecompTotXRequestSoapIn) -> Result<ports::FecompTotXRequestSoapOut, Option<SoapFault>> {

        let __request = FecompTotXRequestSoapInSoapEnvelope::new(SoapFecompTotXRequestSoapIn {
            body: fecomp_tot_x_request_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECompTotXRequest")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecompTotXRequestSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_dummy (&self, fedummy_soap_in: ports::FedummySoapIn) -> Result<ports::FedummySoapOut, Option<SoapFault>> {

        let __request = FedummySoapInSoapEnvelope::new(SoapFedummySoapIn {
            body: fedummy_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEDummy")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FedummySoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_comp_ultimo_autorizado (&self, fecomp_ultimo_autorizado_soap_in: ports::FecompUltimoAutorizadoSoapIn) -> Result<ports::FecompUltimoAutorizadoSoapOut, Option<SoapFault>> {

        let __request = FecompUltimoAutorizadoSoapInSoapEnvelope::new(SoapFecompUltimoAutorizadoSoapIn {
            body: fecomp_ultimo_autorizado_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECompUltimoAutorizado")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecompUltimoAutorizadoSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_comp_consultar (&self, fecomp_consultar_soap_in: ports::FecompConsultarSoapIn) -> Result<ports::FecompConsultarSoapOut, Option<SoapFault>> {

        let __request = FecompConsultarSoapInSoapEnvelope::new(SoapFecompConsultarSoapIn {
            body: fecomp_consultar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECompConsultar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecompConsultarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_reg_informativo (&self, fecaeareg_informativo_soap_in: ports::FecaearegInformativoSoapIn) -> Result<ports::FecaearegInformativoSoapOut, Option<SoapFault>> {

        let __request = FecaearegInformativoSoapInSoapEnvelope::new(SoapFecaearegInformativoSoapIn {
            body: fecaeareg_informativo_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEARegInformativo")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaearegInformativoSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_solicitar (&self, fecaeasolicitar_soap_in: ports::FecaeasolicitarSoapIn) -> Result<ports::FecaeasolicitarSoapOut, Option<SoapFault>> {

        let __request = FecaeasolicitarSoapInSoapEnvelope::new(SoapFecaeasolicitarSoapIn {
            body: fecaeasolicitar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEASolicitar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeasolicitarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_sin_movimiento_consultar (&self, fecaeasin_movimiento_consultar_soap_in: ports::FecaeasinMovimientoConsultarSoapIn) -> Result<ports::FecaeasinMovimientoConsultarSoapOut, Option<SoapFault>> {

        let __request = FecaeasinMovimientoConsultarSoapInSoapEnvelope::new(SoapFecaeasinMovimientoConsultarSoapIn {
            body: fecaeasin_movimiento_consultar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEASinMovimientoConsultar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeasinMovimientoConsultarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_sin_movimiento_informar (&self, fecaeasin_movimiento_informar_soap_in: ports::FecaeasinMovimientoInformarSoapIn) -> Result<ports::FecaeasinMovimientoInformarSoapOut, Option<SoapFault>> {

        let __request = FecaeasinMovimientoInformarSoapInSoapEnvelope::new(SoapFecaeasinMovimientoInformarSoapIn {
            body: fecaeasin_movimiento_informar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEASinMovimientoInformar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeasinMovimientoInformarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fecaea_consultar (&self, fecaeaconsultar_soap_in: ports::FecaeaconsultarSoapIn) -> Result<ports::FecaeaconsultarSoapOut, Option<SoapFault>> {

        let __request = FecaeaconsultarSoapInSoapEnvelope::new(SoapFecaeaconsultarSoapIn {
            body: fecaeaconsultar_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FECAEAConsultar")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FecaeaconsultarSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_cotizacion (&self, feparam_get_cotizacion_soap_in: ports::FeparamGetCotizacionSoapIn) -> Result<ports::FeparamGetCotizacionSoapOut, Option<SoapFault>> {

        let __request = FeparamGetCotizacionSoapInSoapEnvelope::new(SoapFeparamGetCotizacionSoapIn {
            body: feparam_get_cotizacion_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetCotizacion")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetCotizacionSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_tributos (&self, feparam_get_tipos_tributos_soap_in: ports::FeparamGetTiposTributosSoapIn) -> Result<ports::FeparamGetTiposTributosSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposTributosSoapInSoapEnvelope::new(SoapFeparamGetTiposTributosSoapIn {
            body: feparam_get_tipos_tributos_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposTributos")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposTributosSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_monedas (&self, feparam_get_tipos_monedas_soap_in: ports::FeparamGetTiposMonedasSoapIn) -> Result<ports::FeparamGetTiposMonedasSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposMonedasSoapInSoapEnvelope::new(SoapFeparamGetTiposMonedasSoapIn {
            body: feparam_get_tipos_monedas_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposMonedas")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposMonedasSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_iva (&self, feparam_get_tipos_iva_soap_in: ports::FeparamGetTiposIvaSoapIn) -> Result<ports::FeparamGetTiposIvaSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposIvaSoapInSoapEnvelope::new(SoapFeparamGetTiposIvaSoapIn {
            body: feparam_get_tipos_iva_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposIva")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposIvaSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_opcional (&self, feparam_get_tipos_opcional_soap_in: ports::FeparamGetTiposOpcionalSoapIn) -> Result<ports::FeparamGetTiposOpcionalSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposOpcionalSoapInSoapEnvelope::new(SoapFeparamGetTiposOpcionalSoapIn {
            body: feparam_get_tipos_opcional_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposOpcional")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposOpcionalSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_concepto (&self, feparam_get_tipos_concepto_soap_in: ports::FeparamGetTiposConceptoSoapIn) -> Result<ports::FeparamGetTiposConceptoSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposConceptoSoapInSoapEnvelope::new(SoapFeparamGetTiposConceptoSoapIn {
            body: feparam_get_tipos_concepto_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposConcepto")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposConceptoSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_ptos_venta (&self, feparam_get_ptos_venta_soap_in: ports::FeparamGetPtosVentaSoapIn) -> Result<ports::FeparamGetPtosVentaSoapOut, Option<SoapFault>> {

        let __request = FeparamGetPtosVentaSoapInSoapEnvelope::new(SoapFeparamGetPtosVentaSoapIn {
            body: feparam_get_ptos_venta_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetPtosVenta")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetPtosVentaSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_cbte (&self, feparam_get_tipos_cbte_soap_in: ports::FeparamGetTiposCbteSoapIn) -> Result<ports::FeparamGetTiposCbteSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposCbteSoapInSoapEnvelope::new(SoapFeparamGetTiposCbteSoapIn {
            body: feparam_get_tipos_cbte_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposCbte")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposCbteSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_doc (&self, feparam_get_tipos_doc_soap_in: ports::FeparamGetTiposDocSoapIn) -> Result<ports::FeparamGetTiposDocSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposDocSoapInSoapEnvelope::new(SoapFeparamGetTiposDocSoapIn {
            body: feparam_get_tipos_doc_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposDoc")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposDocSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_tipos_paises (&self, feparam_get_tipos_paises_soap_in: ports::FeparamGetTiposPaisesSoapIn) -> Result<ports::FeparamGetTiposPaisesSoapOut, Option<SoapFault>> {

        let __request = FeparamGetTiposPaisesSoapInSoapEnvelope::new(SoapFeparamGetTiposPaisesSoapIn {
            body: feparam_get_tipos_paises_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetTiposPaises")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetTiposPaisesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}	async fn fe_param_get_actividades (&self, feparam_get_actividades_soap_in: ports::FeparamGetActividadesSoapIn) -> Result<ports::FeparamGetActividadesSoapOut, Option<SoapFault>> {

        let __request = FeparamGetActividadesSoapInSoapEnvelope::new(SoapFeparamGetActividadesSoapIn {
            body: feparam_get_actividades_soap_in,
            xmlns: Option::Some("http://ar.gov.afip.dif.FEV1/".to_string()),
        });            
        
        let (status, response) = self.send_soap_request(&__request, "http://ar.gov.afip.dif.FEV1/FEParamGetActividades")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

        let r: FeparamGetActividadesSoapOutSoapEnvelope = from_str(&response).map_err(|err| {
                        warn!("Failed to unmarshal SOAP response: {:?}", err);
                        None
                    })?;
        if status.is_success() {
            Ok(r.body.body)
        } else {
            Err(r.body.fault)
        }}}
}

pub mod services {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            /** Web Service orientado  al  servicio  de Facturacion electronica RG2485 V1
		
 */
pub struct Service {}
               impl Service {
                
            pub fn new_client(credentials: Option<(String, String)>) -> bindings::ServiceSoap {
                bindings::ServiceSoap::new("https://wswhomo.afip.gov.ar/wsfev1/service.asmx", credentials)
            }
        }
}

