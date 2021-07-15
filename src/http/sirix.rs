use sirix_rust_client::synchronous::sirix::Sirix;
use sirix_rust_client::synchronous::auth::auth;

use super::types::JsonResponse;


pub fn create_sirix(base_url: &str, username: &str, password: &str) -> Sirix {
    let agent = ureq::agent();
    let lock = auth(agent.clone(), base_url, username, password);
    Sirix::new(base_url.to_string(), agent.clone(), Some(lock))
}

pub fn server_info(sirix: Sirix) -> JsonResponse {
    match sirix.info_raw() {
        Ok(response) => JsonResponse::Ok(response.body),
        Err(err) => JsonResponse::Err(err),
    }
}

pub fn server_info_with_resources(sirix: Sirix) -> JsonResponse {
    match sirix.info_with_resources_raw() {
        Ok(response) => JsonResponse::Ok(response.body),
        Err(err) => JsonResponse::Err(err),
    }
}