//! Subalfred core HTTP library.

// std
use std::sync::Arc;
// crates.io
use once_cell::sync::Lazy;
use reqwest::Client;

/// Global HTTP client.
pub static CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));
