use sirix_rust_client::{
    synchronous::{client::SirixResponse, database::Database, error::SirixResult},
    types::{Json, Xml},
};

use super::types::{JsonResponse, XmlResponse};

pub fn database_info_json(database: Database<Json>) -> JsonResponse {
    match database.info_raw() {
        Ok(response) => JsonResponse::Ok(response.body),
        Err(err) => JsonResponse::Err(err),
    }
}

pub fn database_info_xml(database: Database<Xml>) -> XmlResponse {
    match database.info_string() {
        Ok(response) => XmlResponse::Ok(response.body),
        Err(err) => XmlResponse::Err(err),
    }
}

pub fn database_delete<T>(database: Database<T>) -> SirixResult<SirixResponse<()>> {
    database.delete()
}
