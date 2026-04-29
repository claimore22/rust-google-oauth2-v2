//! Request executor for Google OAuth2 API v2.
//!
//! Provides shared HTTP request/response handling to eliminate code duplication
//! across API call implementations.

use reqwest::Client;

use crate::error::{OAuth2Error, Result};

/// Shared request executor for all API calls.
/// Eliminates duplication of HTTP request/response handling.
#[derive(Clone)]
pub struct RequestExecutor {
    client: Client,
    user_agent: String,
}

impl RequestExecutor {
    /// Creates a new request executor.
    pub fn new(client: Client, user_agent: String) -> Self {
        Self { client, user_agent }
    }

    /// Returns a clone of the underlying HTTP client.
    pub fn client(&self) -> Client {
        self.client.clone()
    }

    /// Execute GET request and deserialize JSON response.
    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T> {
        let response = self.client
            .get(url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    /// Execute POST request and deserialize JSON response.
    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
    ) -> Result<T> {
        let response = self.client
            .post(url)
            .header("User-Agent", &self.user_agent)
            .send()
            .await?;

        Self::handle_response(response).await
    }

    /// Common response handling logic for all requests.
    async fn handle_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status == reqwest::StatusCode::NOT_MODIFIED {
            return Err(OAuth2Error::NotModified);
        }

        if status.is_success() {
            Ok(response.json::<T>().await?)
        } else {
            let error_text = response.text().await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(OAuth2Error::ApiError(format!("{}: {}", status, error_text)))
        }
    }
}
