use sirix_rust_client::synchronous::sirix::Sirix;
use sirix_rust_client::synchronous::auth::auth;


pub fn create_sirix(base_url: &str, username: &str, password: &str) -> Sirix {
    let agent = ureq::agent();
    let lock = auth(agent.clone(), base_url, username, password);
    Sirix::new(base_url.to_string(), agent.clone(), Some(lock))
}