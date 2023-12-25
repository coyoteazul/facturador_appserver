use yaserde_derive::{YaSerialize,YaDeserialize};
use super::soap_util::{Validate, transport}; //change the address of this USE as needed. "super" works if soap_utils.rs is on the same level as this file



#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaesolicitar {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "FeCAEReq")]
    pub fe_cae_req: Fecaerequest,
}

impl Validate for Fecaesolicitar {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeauthRequest {
    #[yaserde(prefix = "tns", rename = "Token")]
    pub token: String,

    #[yaserde(prefix = "tns", rename = "Sign")]
    pub sign: String,

    #[yaserde(prefix = "tns", rename = "Cuit")]
    pub cuit: i64,
}

impl Validate for FeauthRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaerequest {
    #[yaserde(prefix = "tns", rename = "FeCabReq")]
    pub fe_cab_req: FecaecabRequest,

    #[yaserde(prefix = "tns", rename = "FeDetReq")]
    pub fe_det_req: ArrayOfFECAEDetRequest,
}

impl Validate for Fecaerequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaecabRequest {
    pub base: FecabRequest,
}

impl Validate for FecaecabRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecabRequest {
    #[yaserde(prefix = "tns", rename = "CantReg")]
    pub cant_reg: i32,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: i32,
}

impl Validate for FecabRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfFECAEDetRequest {
    #[yaserde(prefix = "tns", rename = "FECAEDetRequest")]
    pub fecae_det_request: Vec<FecaedetRequest>,
}

impl Validate for ArrayOfFECAEDetRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaedetRequest {
    pub base: FedetRequest,
}

impl Validate for FecaedetRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FedetRequest {
    #[yaserde(prefix = "tns", rename = "Concepto")]
    pub concepto: i32,

    #[yaserde(prefix = "tns", rename = "DocTipo")]
    pub doc_tipo: i32,

    #[yaserde(prefix = "tns", rename = "DocNro")]
    pub doc_nro: i64,

    #[yaserde(prefix = "tns", rename = "CbteDesde")]
    pub cbte_desde: i64,

    #[yaserde(prefix = "tns", rename = "CbteHasta")]
    pub cbte_hasta: i64,

    #[yaserde(prefix = "tns", rename = "CbteFch")]
    pub cbte_fch: String,

    #[yaserde(prefix = "tns", rename = "ImpTotal")]
    pub imp_total: f64,

    #[yaserde(prefix = "tns", rename = "ImpTotConc")]
    pub imp_tot_conc: f64,

    #[yaserde(prefix = "tns", rename = "ImpNeto")]
    pub imp_neto: f64,

    #[yaserde(prefix = "tns", rename = "ImpOpEx")]
    pub imp_op_ex: f64,

    #[yaserde(prefix = "tns", rename = "ImpTrib")]
    pub imp_trib: f64,

    #[yaserde(prefix = "tns", rename = "ImpIVA")]
    pub imp_iva: f64,

    #[yaserde(prefix = "tns", rename = "FchServDesde")]
    pub fch_serv_desde: String,

    #[yaserde(prefix = "tns", rename = "FchServHasta")]
    pub fch_serv_hasta: String,

    #[yaserde(prefix = "tns", rename = "FchVtoPago")]
    pub fch_vto_pago: String,

    #[yaserde(prefix = "tns", rename = "MonId")]
    pub mon_id: String,

    #[yaserde(prefix = "tns", rename = "MonCotiz")]
    pub mon_cotiz: f64,

    #[yaserde(prefix = "tns", rename = "CbtesAsoc")]
    pub cbtes_asoc: ArrayOfCbteAsoc,

    #[yaserde(prefix = "tns", rename = "Tributos")]
    pub tributos: ArrayOfTributo,

    #[yaserde(prefix = "tns", rename = "Iva")]
    pub iva: ArrayOfAlicIva,

    #[yaserde(prefix = "tns", rename = "Opcionales")]
    pub opcionales: ArrayOfOpcional,

    #[yaserde(prefix = "tns", rename = "Compradores")]
    pub compradores: ArrayOfComprador,

    #[yaserde(prefix = "tns", rename = "PeriodoAsoc")]
    pub periodo_asoc: Periodo,

    #[yaserde(prefix = "tns", rename = "Actividades")]
    pub actividades: ArrayOfActividad,
}

impl Validate for FedetRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfCbteAsoc {
    #[yaserde(prefix = "tns", rename = "CbteAsoc")]
    pub cbte_asoc: Vec<CbteAsoc>,
}

impl Validate for ArrayOfCbteAsoc {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct CbteAsoc {
    #[yaserde(prefix = "tns", rename = "Tipo")]
    pub tipo: i32,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "Nro")]
    pub nro: i64,

    #[yaserde(prefix = "tns", rename = "Cuit")]
    pub cuit: String,

    #[yaserde(prefix = "tns", rename = "CbteFch")]
    pub cbte_fch: String,
}

impl Validate for CbteAsoc {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfTributo {
    #[yaserde(prefix = "tns", rename = "Tributo")]
    pub tributo: Vec<Tributo>,
}

impl Validate for ArrayOfTributo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Tributo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i16,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "BaseImp")]
    pub base_imp: f64,

    #[yaserde(prefix = "tns", rename = "Alic")]
    pub alic: f64,

    #[yaserde(prefix = "tns", rename = "Importe")]
    pub importe: f64,
}

impl Validate for Tributo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfAlicIva {
    #[yaserde(prefix = "tns", rename = "AlicIva")]
    pub alic_iva: Vec<AlicIva>,
}

impl Validate for ArrayOfAlicIva {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct AlicIva {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i32,

    #[yaserde(prefix = "tns", rename = "BaseImp")]
    pub base_imp: f64,

    #[yaserde(prefix = "tns", rename = "Importe")]
    pub importe: f64,
}

impl Validate for AlicIva {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfOpcional {
    #[yaserde(prefix = "tns", rename = "Opcional")]
    pub opcional: Vec<Opcional>,
}

impl Validate for ArrayOfOpcional {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Opcional {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: String,

    #[yaserde(prefix = "tns", rename = "Valor")]
    pub valor: String,
}

impl Validate for Opcional {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfComprador {
    #[yaserde(prefix = "tns", rename = "Comprador")]
    pub comprador: Vec<Comprador>,
}

impl Validate for ArrayOfComprador {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Comprador {
    #[yaserde(prefix = "tns", rename = "DocTipo")]
    pub doc_tipo: i32,

    #[yaserde(prefix = "tns", rename = "DocNro")]
    pub doc_nro: i64,

    #[yaserde(prefix = "tns", rename = "Porcentaje")]
    pub porcentaje: f64,
}

impl Validate for Comprador {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Periodo {
    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for Periodo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfActividad {
    #[yaserde(prefix = "tns", rename = "Actividad")]
    pub actividad: Vec<Actividad>,
}

impl Validate for ArrayOfActividad {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Actividad {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i64,
}

impl Validate for Actividad {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaesolicitarResponse {
    #[yaserde(prefix = "tns", rename = "FECAESolicitarResult")]
    pub fecae_solicitar_result: Fecaeresponse,
}

impl Validate for FecaesolicitarResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaeresponse {
    #[yaserde(prefix = "tns", rename = "FeCabResp")]
    pub fe_cab_resp: FecaecabResponse,

    #[yaserde(prefix = "tns", rename = "FeDetResp")]
    pub fe_det_resp: ArrayOfFECAEDetResponse,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,
}

impl Validate for Fecaeresponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaecabResponse {
    pub base: FecabResponse,
}

impl Validate for FecaecabResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecabResponse {
    #[yaserde(prefix = "tns", rename = "Cuit")]
    pub cuit: i64,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: i32,

    #[yaserde(prefix = "tns", rename = "FchProceso")]
    pub fch_proceso: String,

    #[yaserde(prefix = "tns", rename = "CantReg")]
    pub cant_reg: i32,

    #[yaserde(prefix = "tns", rename = "Resultado")]
    pub resultado: String,

    #[yaserde(prefix = "tns", rename = "Reproceso")]
    pub reproceso: String,
}

impl Validate for FecabResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfFECAEDetResponse {
    #[yaserde(prefix = "tns", rename = "FECAEDetResponse")]
    pub fecae_det_response: Vec<FecaedetResponse>,
}

impl Validate for ArrayOfFECAEDetResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaedetResponse {
    #[yaserde(prefix = "tns", rename = "CAE")]
    pub cae: String,

    #[yaserde(prefix = "tns", rename = "CAEFchVto")]
    pub cae_fch_vto: String,

    pub base: FedetResponse,
}

impl Validate for FecaedetResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FedetResponse {
    #[yaserde(prefix = "tns", rename = "Concepto")]
    pub concepto: i32,

    #[yaserde(prefix = "tns", rename = "DocTipo")]
    pub doc_tipo: i32,

    #[yaserde(prefix = "tns", rename = "DocNro")]
    pub doc_nro: i64,

    #[yaserde(prefix = "tns", rename = "CbteDesde")]
    pub cbte_desde: i64,

    #[yaserde(prefix = "tns", rename = "CbteHasta")]
    pub cbte_hasta: i64,

    #[yaserde(prefix = "tns", rename = "CbteFch")]
    pub cbte_fch: String,

    #[yaserde(prefix = "tns", rename = "Resultado")]
    pub resultado: String,

    #[yaserde(prefix = "tns", rename = "Observaciones")]
    pub observaciones: ArrayOfObs,
}

impl Validate for FedetResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfObs {
    #[yaserde(prefix = "tns", rename = "Obs")]
    pub obs: Vec<Obs>,
}

impl Validate for ArrayOfObs {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Obs {
    #[yaserde(prefix = "tns", rename = "Code")]
    pub code: i32,

    #[yaserde(prefix = "tns", rename = "Msg")]
    pub msg: String,
}

impl Validate for Obs {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfEvt {
    #[yaserde(prefix = "tns", rename = "Evt")]
    pub evt: Vec<Evt>,
}

impl Validate for ArrayOfEvt {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Evt {
    #[yaserde(prefix = "tns", rename = "Code")]
    pub code: i32,

    #[yaserde(prefix = "tns", rename = "Msg")]
    pub msg: String,
}

impl Validate for Evt {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfErr {
    #[yaserde(prefix = "tns", rename = "Err")]
    pub err: Vec<Err>,
}

impl Validate for ArrayOfErr {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Err {
    #[yaserde(prefix = "tns", rename = "Code")]
    pub code: i32,

    #[yaserde(prefix = "tns", rename = "Msg")]
    pub msg: String,
}

impl Validate for Err {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompTotXRequest {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FecompTotXRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompTotXRequestResponse {
    #[yaserde(prefix = "tns", rename = "FECompTotXRequestResult")]
    pub fe_comp_tot_x_request_result: FeregXReqResponse,
}

impl Validate for FecompTotXRequestResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeregXReqResponse {
    #[yaserde(prefix = "tns", rename = "RegXReq")]
    pub reg_x_req: i32,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FeregXReqResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fedummy {}

impl Validate for Fedummy {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FedummyResponse {
    #[yaserde(prefix = "tns", rename = "FEDummyResult")]
    pub fe_dummy_result: DummyResponse,
}

impl Validate for FedummyResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct DummyResponse {
    #[yaserde(prefix = "tns", rename = "AppServer")]
    pub app_server: String,

    #[yaserde(prefix = "tns", rename = "DbServer")]
    pub db_server: String,

    #[yaserde(prefix = "tns", rename = "AuthServer")]
    pub auth_server: String,
}

impl Validate for DummyResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompUltimoAutorizado {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: i32,
}

impl Validate for FecompUltimoAutorizado {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompUltimoAutorizadoResponse {
    #[yaserde(prefix = "tns", rename = "FECompUltimoAutorizadoResult")]
    pub fe_comp_ultimo_autorizado_result: FerecuperaLastCbteResponse,
}

impl Validate for FecompUltimoAutorizadoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FerecuperaLastCbteResponse {
    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: i32,

    #[yaserde(prefix = "tns", rename = "CbteNro")]
    pub cbte_nro: i32,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FerecuperaLastCbteResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompConsultar {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "FeCompConsReq")]
    pub fe_comp_cons_req: FecompConsultaReq,
}

impl Validate for FecompConsultar {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompConsultaReq {
    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: i32,

    #[yaserde(prefix = "tns", rename = "CbteNro")]
    pub cbte_nro: i64,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,
}

impl Validate for FecompConsultaReq {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompConsultarResponse {
    #[yaserde(prefix = "tns", rename = "FECompConsultarResult")]
    pub fe_comp_consultar_result: FecompConsultaResponse,
}

impl Validate for FecompConsultarResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompConsultaResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: FecompConsResponse,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FecompConsultaResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecompConsResponse {
    #[yaserde(prefix = "tns", rename = "Resultado")]
    pub resultado: String,

    #[yaserde(prefix = "tns", rename = "CodAutorizacion")]
    pub cod_autorizacion: String,

    #[yaserde(prefix = "tns", rename = "EmisionTipo")]
    pub emision_tipo: String,

    #[yaserde(prefix = "tns", rename = "FchVto")]
    pub fch_vto: String,

    #[yaserde(prefix = "tns", rename = "FchProceso")]
    pub fch_proceso: String,

    #[yaserde(prefix = "tns", rename = "Observaciones")]
    pub observaciones: ArrayOfObs,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: i32,

    pub base: FecaedetRequest,
}

impl Validate for FecompConsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaearegInformativo {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "FeCAEARegInfReq")]
    pub fe_caea_reg_inf_req: Fecaearequest,
}

impl Validate for FecaearegInformativo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaearequest {
    #[yaserde(prefix = "tns", rename = "FeCabReq")]
    pub fe_cab_req: FecaeacabRequest,

    #[yaserde(prefix = "tns", rename = "FeDetReq")]
    pub fe_det_req: ArrayOfFECAEADetRequest,
}

impl Validate for Fecaearequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeacabRequest {
    pub base: FecabRequest,
}

impl Validate for FecaeacabRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfFECAEADetRequest {
    #[yaserde(prefix = "tns", rename = "FECAEADetRequest")]
    pub fecaea_det_request: Vec<FecaeadetRequest>,
}

impl Validate for ArrayOfFECAEADetRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeadetRequest {
    #[yaserde(prefix = "tns", rename = "CAEA")]
    pub caea: String,

    #[yaserde(prefix = "tns", rename = "CbteFchHsGen")]
    pub cbte_fch_hs_gen: String,

    pub base: FedetRequest,
}

impl Validate for FecaeadetRequest {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaearegInformativoResponse {
    #[yaserde(prefix = "tns", rename = "FECAEARegInformativoResult")]
    pub fecaea_reg_informativo_result: Fecaearesponse,
}

impl Validate for FecaearegInformativoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaearesponse {
    #[yaserde(prefix = "tns", rename = "FeCabResp")]
    pub fe_cab_resp: FecaeacabResponse,

    #[yaserde(prefix = "tns", rename = "FeDetResp")]
    pub fe_det_resp: ArrayOfFECAEADetResponse,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,
}

impl Validate for Fecaearesponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeacabResponse {
    pub base: FecabResponse,
}

impl Validate for FecaeacabResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfFECAEADetResponse {
    #[yaserde(prefix = "tns", rename = "FECAEADetResponse")]
    pub fecaea_det_response: Vec<FecaeadetResponse>,
}

impl Validate for ArrayOfFECAEADetResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeadetResponse {
    #[yaserde(prefix = "tns", rename = "CAEA")]
    pub caea: String,

    pub base: FedetResponse,
}

impl Validate for FecaeadetResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaeasolicitar {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "Periodo")]
    pub periodo: i32,

    #[yaserde(prefix = "tns", rename = "Orden")]
    pub orden: i16,
}

impl Validate for Fecaeasolicitar {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasolicitarResponse {
    #[yaserde(prefix = "tns", rename = "FECAEASolicitarResult")]
    pub fecaea_solicitar_result: FecaeagetResponse,
}

impl Validate for FecaeasolicitarResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeagetResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: Fecaeaget,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FecaeagetResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaeaget {
    #[yaserde(prefix = "tns", rename = "CAEA")]
    pub caea: String,

    #[yaserde(prefix = "tns", rename = "Periodo")]
    pub periodo: i32,

    #[yaserde(prefix = "tns", rename = "Orden")]
    pub orden: i16,

    #[yaserde(prefix = "tns", rename = "FchVigDesde")]
    pub fch_vig_desde: String,

    #[yaserde(prefix = "tns", rename = "FchVigHasta")]
    pub fch_vig_hasta: String,

    #[yaserde(prefix = "tns", rename = "FchTopeInf")]
    pub fch_tope_inf: String,

    #[yaserde(prefix = "tns", rename = "FchProceso")]
    pub fch_proceso: String,

    #[yaserde(prefix = "tns", rename = "Observaciones")]
    pub observaciones: ArrayOfObs,
}

impl Validate for Fecaeaget {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMovimientoConsultar {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "CAEA")]
    pub caea: String,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,
}

impl Validate for FecaeasinMovimientoConsultar {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMovimientoConsultarResponse {
    #[yaserde(prefix = "tns", rename = "FECAEASinMovimientoConsultarResult")]
    pub fecaea_sin_movimiento_consultar_result: FecaeasinMovConsResponse,
}

impl Validate for FecaeasinMovimientoConsultarResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMovConsResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfFECAEASinMov,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FecaeasinMovConsResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfFECAEASinMov {
    #[yaserde(prefix = "tns", rename = "FECAEASinMov")]
    pub fecaea_sin_mov: Vec<FecaeasinMov>,
}

impl Validate for ArrayOfFECAEASinMov {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMov {
    #[yaserde(prefix = "tns", rename = "CAEA")]
    pub caea: String,

    #[yaserde(prefix = "tns", rename = "FchProceso")]
    pub fch_proceso: String,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,
}

impl Validate for FecaeasinMov {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMovimientoInformar {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "PtoVta")]
    pub pto_vta: i32,

    #[yaserde(prefix = "tns", rename = "CAEA")]
    pub caea: String,
}

impl Validate for FecaeasinMovimientoInformar {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMovimientoInformarResponse {
    #[yaserde(prefix = "tns", rename = "FECAEASinMovimientoInformarResult")]
    pub fecaea_sin_movimiento_informar_result: FecaeasinMovResponse,
}

impl Validate for FecaeasinMovimientoInformarResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeasinMovResponse {
    #[yaserde(prefix = "tns", rename = "Resultado")]
    pub resultado: String,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,

    pub base: FecaeasinMov,
}

impl Validate for FecaeasinMovResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Fecaeaconsultar {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "Periodo")]
    pub periodo: i32,

    #[yaserde(prefix = "tns", rename = "Orden")]
    pub orden: i16,
}

impl Validate for Fecaeaconsultar {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecaeaconsultarResponse {
    #[yaserde(prefix = "tns", rename = "FECAEAConsultarResult")]
    pub fecaea_consultar_result: FecaeagetResponse,
}

impl Validate for FecaeaconsultarResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetCotizacion {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,

    #[yaserde(prefix = "tns", rename = "MonId")]
    pub mon_id: String,
}

impl Validate for FeparamGetCotizacion {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetCotizacionResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetCotizacionResult")]
    pub fe_param_get_cotizacion_result: FecotizacionResponse,
}

impl Validate for FeparamGetCotizacionResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FecotizacionResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: Cotizacion,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FecotizacionResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Cotizacion {
    #[yaserde(prefix = "tns", rename = "MonId")]
    pub mon_id: String,

    #[yaserde(prefix = "tns", rename = "MonCotiz")]
    pub mon_cotiz: f64,

    #[yaserde(prefix = "tns", rename = "FchCotiz")]
    pub fch_cotiz: String,
}

impl Validate for Cotizacion {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposTributos {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposTributos {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposTributosResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposTributosResult")]
    pub fe_param_get_tipos_tributos_result: FetributoResponse,
}

impl Validate for FeparamGetTiposTributosResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FetributoResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfTributoTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FetributoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfTributoTipo {
    #[yaserde(prefix = "tns", rename = "TributoTipo")]
    pub tributo_tipo: Vec<TributoTipo>,
}

impl Validate for ArrayOfTributoTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct TributoTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i16,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for TributoTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposMonedas {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposMonedas {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposMonedasResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposMonedasResult")]
    pub fe_param_get_tipos_monedas_result: MonedaResponse,
}

impl Validate for FeparamGetTiposMonedasResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct MonedaResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfMoneda,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for MonedaResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfMoneda {
    #[yaserde(prefix = "tns", rename = "Moneda")]
    pub moneda: Vec<Moneda>,
}

impl Validate for ArrayOfMoneda {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct Moneda {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: String,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for Moneda {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposIva {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposIva {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposIvaResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposIvaResult")]
    pub fe_param_get_tipos_iva_result: IvaTipoResponse,
}

impl Validate for FeparamGetTiposIvaResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct IvaTipoResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfIvaTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for IvaTipoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfIvaTipo {
    #[yaserde(prefix = "tns", rename = "IvaTipo")]
    pub iva_tipo: Vec<IvaTipo>,
}

impl Validate for ArrayOfIvaTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct IvaTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: String,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for IvaTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposOpcional {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposOpcional {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposOpcionalResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposOpcionalResult")]
    pub fe_param_get_tipos_opcional_result: OpcionalTipoResponse,
}

impl Validate for FeparamGetTiposOpcionalResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct OpcionalTipoResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfOpcionalTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for OpcionalTipoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfOpcionalTipo {
    #[yaserde(prefix = "tns", rename = "OpcionalTipo")]
    pub opcional_tipo: Vec<OpcionalTipo>,
}

impl Validate for ArrayOfOpcionalTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct OpcionalTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: String,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for OpcionalTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposConcepto {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposConcepto {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposConceptoResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposConceptoResult")]
    pub fe_param_get_tipos_concepto_result: ConceptoTipoResponse,
}

impl Validate for FeparamGetTiposConceptoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ConceptoTipoResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfConceptoTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for ConceptoTipoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfConceptoTipo {
    #[yaserde(prefix = "tns", rename = "ConceptoTipo")]
    pub concepto_tipo: Vec<ConceptoTipo>,
}

impl Validate for ArrayOfConceptoTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ConceptoTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i32,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for ConceptoTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetPtosVenta {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetPtosVenta {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetPtosVentaResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetPtosVentaResult")]
    pub fe_param_get_ptos_venta_result: FeptoVentaResponse,
}

impl Validate for FeparamGetPtosVentaResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeptoVentaResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfPtoVenta,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FeptoVentaResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfPtoVenta {
    #[yaserde(prefix = "tns", rename = "PtoVenta")]
    pub pto_venta: Vec<PtoVenta>,
}

impl Validate for ArrayOfPtoVenta {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct PtoVenta {
    #[yaserde(prefix = "tns", rename = "Nro")]
    pub nro: i32,

    #[yaserde(prefix = "tns", rename = "EmisionTipo")]
    pub emision_tipo: String,

    #[yaserde(prefix = "tns", rename = "Bloqueado")]
    pub bloqueado: String,

    #[yaserde(prefix = "tns", rename = "FchBaja")]
    pub fch_baja: String,
}

impl Validate for PtoVenta {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposCbte {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposCbte {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposCbteResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposCbteResult")]
    pub fe_param_get_tipos_cbte_result: CbteTipoResponse,
}

impl Validate for FeparamGetTiposCbteResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct CbteTipoResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfCbteTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for CbteTipoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfCbteTipo {
    #[yaserde(prefix = "tns", rename = "CbteTipo")]
    pub cbte_tipo: Vec<CbteTipo>,
}

impl Validate for ArrayOfCbteTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct CbteTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i32,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for CbteTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposDoc {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposDoc {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposDocResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposDocResult")]
    pub fe_param_get_tipos_doc_result: DocTipoResponse,
}

impl Validate for FeparamGetTiposDocResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct DocTipoResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfDocTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for DocTipoResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfDocTipo {
    #[yaserde(prefix = "tns", rename = "DocTipo")]
    pub doc_tipo: Vec<DocTipo>,
}

impl Validate for ArrayOfDocTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct DocTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i32,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,

    #[yaserde(prefix = "tns", rename = "FchDesde")]
    pub fch_desde: String,

    #[yaserde(prefix = "tns", rename = "FchHasta")]
    pub fch_hasta: String,
}

impl Validate for DocTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposPaises {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetTiposPaises {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetTiposPaisesResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetTiposPaisesResult")]
    pub fe_param_get_tipos_paises_result: FepaisResponse,
}

impl Validate for FeparamGetTiposPaisesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FepaisResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfPaisTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FepaisResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfPaisTipo {
    #[yaserde(prefix = "tns", rename = "PaisTipo")]
    pub pais_tipo: Vec<PaisTipo>,
}

impl Validate for ArrayOfPaisTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct PaisTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i16,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,
}

impl Validate for PaisTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetActividades {
    #[yaserde(prefix = "tns", rename = "Auth")]
    pub auth: FeauthRequest,
}

impl Validate for FeparamGetActividades {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeparamGetActividadesResponse {
    #[yaserde(prefix = "tns", rename = "FEParamGetActividadesResult")]
    pub fe_param_get_actividades_result: FeactividadesResponse,
}

impl Validate for FeparamGetActividadesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct FeactividadesResponse {
    #[yaserde(prefix = "tns", rename = "ResultGet")]
    pub result_get: ArrayOfActividadesTipo,

    #[yaserde(prefix = "tns", rename = "Errors")]
    pub errors: ArrayOfErr,

    #[yaserde(prefix = "tns", rename = "Events")]
    pub events: ArrayOfEvt,
}

impl Validate for FeactividadesResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ArrayOfActividadesTipo {
    #[yaserde(prefix = "tns", rename = "ActividadesTipo")]
    pub actividades_tipo: Vec<ActividadesTipo>,
}

impl Validate for ArrayOfActividadesTipo {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://ar.gov.afip.dif.FEV1/")]
pub struct ActividadesTipo {
    #[yaserde(prefix = "tns", rename = "Id")]
    pub id: i64,

    #[yaserde(prefix = "tns", rename = "Orden")]
    pub orden: i16,

    #[yaserde(prefix = "tns", rename = "Desc")]
    pub desc: String,
}

impl Validate for ActividadesTipo {}



// Solicitud de Código de Autorización Electrónico (CAE)
pub async fn fecae_solicitar<T: transport::Transport>(
    transport: &T,
    request: &Fecaesolicitar
) -> Result<FecaesolicitarResponse, transport::Error> {
    transport::request(transport, request).await
}

// Retorna la cantidad maxima de registros que puede tener una invocacion al
// metodo FECAESolicitar / FECAEARegInformativo
pub async fn fe_comp_tot_x_request<T: transport::Transport>(
    transport: &T,
    request: &FecompTotXRequest
) -> Result<FecompTotXRequestResponse, transport::Error> {
    transport::request(transport, request).await
}

// Metodo dummy para verificacion de funcionamiento
pub async fn fe_dummy<T: transport::Transport>(
    transport: &T,
    request: &Fedummy
) -> Result<FedummyResponse, transport::Error> {
    transport::request(transport, request).await
}

// Retorna el ultimo comprobante autorizado para el tipo de comprobante / cuit /
// punto de venta ingresado / Tipo de Emisión
pub async fn fe_comp_ultimo_autorizado<T: transport::Transport>(
    transport: &T,
    request: &FecompUltimoAutorizado
) -> Result<FecompUltimoAutorizadoResponse, transport::Error> {
    transport::request(transport, request).await
}

// Consulta Comprobante emitido y su código.
pub async fn fe_comp_consultar<T: transport::Transport>(
    transport: &T,
    request: &FecompConsultar
) -> Result<FecompConsultarResponse, transport::Error> {
    transport::request(transport, request).await
}

// Rendición de comprobantes asociados a un CAEA.
pub async fn fecaea_reg_informativo<T: transport::Transport>(
    transport: &T,
    request: &FecaearegInformativo
) -> Result<FecaearegInformativoResponse, transport::Error> {
    transport::request(transport, request).await
}

// Solicitud de Código de Autorización Electrónico Anticipado (CAEA)
pub async fn fecaea_solicitar<T: transport::Transport>(
    transport: &T,
    request: &Fecaeasolicitar
) -> Result<FecaeasolicitarResponse, transport::Error> {
    transport::request(transport, request).await
}

// Consulta CAEA informado como sin movimientos.
pub async fn fecaea_sin_movimiento_consultar<T: transport::Transport>(
    transport: &T,
    request: &FecaeasinMovimientoConsultar
) -> Result<FecaeasinMovimientoConsultarResponse, transport::Error> {
    transport::request(transport, request).await
}

// Informa CAEA sin movimientos.
pub async fn fecaea_sin_movimiento_informar<T: transport::Transport>(
    transport: &T,
    request: &FecaeasinMovimientoInformar
) -> Result<FecaeasinMovimientoInformarResponse, transport::Error> {
    transport::request(transport, request).await
}

// Consultar CAEA emitidos.
pub async fn fecaea_consultar<T: transport::Transport>(
    transport: &T,
    request: &Fecaeaconsultar
) -> Result<FecaeaconsultarResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera la cotizacion de la moneda consultada y su fecha
pub async fn fe_param_get_cotizacion<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetCotizacion
) -> Result<FeparamGetCotizacionResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de los diferente tributos que pueden ser utilizados en el
// servicio de autorizacion
pub async fn fe_param_get_tipos_tributos<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposTributos
) -> Result<FeparamGetTiposTributosResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de monedas utilizables en servicio de autorización
pub async fn fe_param_get_tipos_monedas<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposMonedas
) -> Result<FeparamGetTiposMonedasResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de Tipos de Iva utilizables en servicio de autorización.
pub async fn fe_param_get_tipos_iva<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposIva
) -> Result<FeparamGetTiposIvaResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de identificadores para los campos Opcionales
pub async fn fe_param_get_tipos_opcional<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposOpcional
) -> Result<FeparamGetTiposOpcionalResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de identificadores para el campo Concepto.
pub async fn fe_param_get_tipos_concepto<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposConcepto
) -> Result<FeparamGetTiposConceptoResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de puntos de venta registrados y su estado
pub async fn fe_param_get_ptos_venta<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetPtosVenta
) -> Result<FeparamGetPtosVentaResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de Tipos de Comprobantes utilizables en servicio de
// autorización.
pub async fn fe_param_get_tipos_cbte<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposCbte
) -> Result<FeparamGetTiposCbteResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de Tipos de Documentos utilizables en servicio de
// autorización.
pub async fn fe_param_get_tipos_doc<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposDoc
) -> Result<FeparamGetTiposDocResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de los diferente paises que pueden ser utilizados en el
// servicio de autorizacion
pub async fn fe_param_get_tipos_paises<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetTiposPaises
) -> Result<FeparamGetTiposPaisesResponse, transport::Error> {
    transport::request(transport, request).await
}

// Recupera el listado de las diferentes actividades habilitadas para el emisor
pub async fn fe_param_get_actividades<T: transport::Transport>(
    transport: &T,
    request: &FeparamGetActividades
) -> Result<FeparamGetActividadesResponse, transport::Error> {
    transport::request(transport, request).await
}
