use sirix_rust_client::synchronous::error::SirixError;

pub fn handle_error(err: SirixError) {
    match err {
        SirixError::ConnectionError(err) => match err {
            ureq::Error::Transport(err) => {
                println!("Transport error: {}", err);
            }
            ureq::Error::Status(status, response) => {
                println!("Status: {}, {}", status, response.into_string().unwrap());
            }
        },
        SirixError::FormatError(err) => {
            println!("Format error: {}", err);
        }
    }
}
