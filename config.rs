//! Configuration constants for Google OAuth2 API v2.

/// API identification constants
pub const API_ID: &str = "oauth2:v2";
pub const API_NAME: &str = "oauth2";
pub const API_VERSION: &str = "v2";

/// Base URL paths
pub const BASE_PATH: &str = "https://www.googleapis.com/";
pub const BASE_PATH_TEMPLATE: &str = "https://www.UNIVERSE_DOMAIN/";

/// OAuth2 scope constants
pub const USERINFO_EMAIL_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.email";
pub const USERINFO_PROFILE_SCOPE: &str = "https://www.googleapis.com/auth/userinfo.profile";
pub const OPENID_SCOPE: &str = "openid";
