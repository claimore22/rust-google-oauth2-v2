//! Google OAuth2 API v2 Client
//!
//! This library provides access to the Google OAuth2 API v2.
//!
//! # Example
//!
//! ```no_run
//! use google_oauth2_v2::Service;
//! use reqwest::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     let service = Service::new(client);
//!
//!     // Get tokeninfo
//!     let tokeninfo = service.tokeninfo()
//!         .access_token("your_access_token".to_string())
//!         .do_call()
//!         .await?;
//!
//!     tracing::info!("Tokeninfo: {:?}", tokeninfo);
//!     Ok(())
//! }
//! ```

pub mod calls;
pub mod config;
pub mod error;
pub mod executor;
pub mod models;
pub mod service;

pub use calls::{TokeninfoCall, UserinfoGetCall, UserinfoV2MeGetCall, ServiceState};
pub use config::{
    API_ID, API_NAME, API_VERSION, BASE_PATH, BASE_PATH_TEMPLATE,
    USERINFO_EMAIL_SCOPE, USERINFO_PROFILE_SCOPE, OPENID_SCOPE,
};
pub use error::{OAuth2Error, Result};
pub use models::{Tokeninfo, Userinfo};
pub use service::Service;
