pub mod error;
pub mod resource;
pub mod sirix;
pub mod types;

pub use resource::{read_json_resource, read_xml_resource};
pub use sirix::create_sirix;
pub use error::handle_error;