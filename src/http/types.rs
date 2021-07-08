//use minidom::Element;
use serde_json::value::Value;
use sirix_rust_client::synchronous::error::SirixError;

pub enum JsonResponse {
    Ok(Value),
    Err(SirixError),
}

pub enum XmlResponse {
    Ok(String),
    Err(SirixError),
}
