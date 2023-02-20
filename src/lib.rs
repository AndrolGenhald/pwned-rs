#[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
compile_error!("either native-tls or rustls-tls is required");

mod model;

pub mod errors;
pub mod api;
