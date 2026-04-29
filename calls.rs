//! API call implementations for Google OAuth2 API v2.
//!
//! Each call type handles URL building and delegates HTTP execution to RequestExecutor.

use std::sync::Arc;

use crate::config::BASE_PATH;
use crate::executor::RequestExecutor;
use crate::models::{Tokeninfo, Userinfo};
use crate::error::Result;

/// Shared service state for all API calls.
#[derive(Clone)]
pub struct ServiceState {
    pub executor: RequestExecutor,
    pub base_path: String,
}

impl ServiceState {
    /// Creates new service state with default base path.
    pub fn new(executor: RequestExecutor) -> Self {
        Self {
            executor,
            base_path: BASE_PATH.to_string(),
        }
    }

    /// Creates new service state with custom base path.
    pub fn with_base_path(self, base_path: String) -> Self {
        Self { base_path, ..self }
    }

    /// Creates new service state with custom user agent.
    pub fn with_user_agent(self, user_agent: String) -> Self {
        let client = self.executor.client();
        Self {
            executor: RequestExecutor::new(client, user_agent),
            base_path: self.base_path,
        }
    }
}

/// Builder for tokeninfo API call.
pub struct TokeninfoCall {
    state: Arc<ServiceState>,
    access_token: Option<String>,
    id_token: Option<String>,
    fields: Option<String>,
}

impl TokeninfoCall {
    pub(crate) fn new(state: Arc<ServiceState>) -> Self {
        Self {
            state,
            access_token: None,
            id_token: None,
            fields: None,
        }
    }

    pub fn access_token(mut self, token: String) -> Self {
        self.access_token = Some(token);
        self
    }

    pub fn id_token(mut self, token: String) -> Self {
        self.id_token = Some(token);
        self
    }

    pub fn fields(mut self, fields: String) -> Self {
        self.fields = Some(fields);
        self
    }

    pub async fn do_call(self) -> Result<Tokeninfo> {
        let mut url = url::Url::parse(&format!("{}oauth2/v2/tokeninfo", self.state.base_path))?;

        if let Some(token) = &self.access_token {
            url.query_pairs_mut().append_pair("access_token", token);
        }

        if let Some(token) = &self.id_token {
            url.query_pairs_mut().append_pair("id_token", token);
        }

        if let Some(fields) = &self.fields {
            url.query_pairs_mut().append_pair("fields", fields);
        }

        self.state.executor.post(url.as_str()).await
    }
}

/// Builder for userinfo API call.
pub struct UserinfoGetCall {
    state: Arc<ServiceState>,
    fields: Option<String>,
    if_none_match: Option<String>,
}

impl UserinfoGetCall {
    pub(crate) fn new(state: Arc<ServiceState>) -> Self {
        Self {
            state,
            fields: None,
            if_none_match: None,
        }
    }

    pub fn fields(mut self, fields: String) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn if_none_match(mut self, entity_tag: String) -> Self {
        self.if_none_match = Some(entity_tag);
        self
    }

    pub async fn do_call(self) -> Result<Userinfo> {
        let mut url = url::Url::parse(&format!("{}oauth2/v2/userinfo", self.state.base_path))?;

        if let Some(fields) = &self.fields {
            url.query_pairs_mut().append_pair("fields", fields);
        }

        url.query_pairs_mut().append_pair("alt", "json");
        url.query_pairs_mut().append_pair("prettyPrint", "false");

        self.state.executor.get(url.as_str()).await
    }
}

/// Builder for userinfo v2 me API call.
pub struct UserinfoV2MeGetCall {
    state: Arc<ServiceState>,
    fields: Option<String>,
    if_none_match: Option<String>,
}

impl UserinfoV2MeGetCall {
    pub(crate) fn new(state: Arc<ServiceState>) -> Self {
        Self {
            state,
            fields: None,
            if_none_match: None,
        }
    }

    pub fn fields(mut self, fields: String) -> Self {
        self.fields = Some(fields);
        self
    }

    pub fn if_none_match(mut self, entity_tag: String) -> Self {
        self.if_none_match = Some(entity_tag);
        self
    }

    pub async fn do_call(self) -> Result<Userinfo> {
        let mut url = url::Url::parse(&format!("{}userinfo/v2/me", self.state.base_path))?;

        if let Some(fields) = &self.fields {
            url.query_pairs_mut().append_pair("fields", fields);
        }

        url.query_pairs_mut().append_pair("alt", "json");
        url.query_pairs_mut().append_pair("prettyPrint", "false");

        self.state.executor.get(url.as_str()).await
    }
}
