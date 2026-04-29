//! Service layer for Google OAuth2 API v2.
//!
//! Provides a flattened API surface for easy endpoint navigation.

use std::sync::Arc;

pub use crate::calls::{ServiceState, TokeninfoCall, UserinfoGetCall, UserinfoV2MeGetCall};

/// Main entry point for Google OAuth2 API v2.
#[derive(Clone)]
pub struct Service {
    state: Arc<ServiceState>,
}

impl Service {
    /// Creates a new service with default configuration.
    pub fn new(client: reqwest::Client) -> Self {
        let state = Arc::new(ServiceState::new(crate::executor::RequestExecutor::new(
            client,
            "google-api-rs-client/1.0".to_string(),
        )));
        Self { state }
    }

    /// Sets a custom base path for API requests.
    pub fn with_base_path(self, base_path: String) -> Self {
        let state = Arc::new(ServiceState::with_base_path(
            (*self.state).clone(),
            base_path,
        ));
        Self { state }
    }

    /// Sets a custom user agent for API requests.
    pub fn with_user_agent(self, user_agent: String) -> Self {
        let state = Arc::new(ServiceState::with_user_agent(
            (*self.state).clone(),
            user_agent,
        ));
        Self { state }
    }

    /// Returns a builder for tokeninfo API calls.
    pub fn tokeninfo(&self) -> TokeninfoCall {
        TokeninfoCall::new(self.state.clone())
    }

    /// Returns a builder for userinfo API calls (v2/me).
    pub fn userinfo_v2_me(&self) -> UserinfoV2MeGetCall {
        UserinfoV2MeGetCall::new(self.state.clone())
    }

    /// Returns a builder for userinfo API calls.
    pub fn userinfo(&self) -> UserinfoGetCall {
        UserinfoGetCall::new(self.state.clone())
    }
}
