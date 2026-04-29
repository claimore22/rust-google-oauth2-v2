# Google OAuth2 API v2 Client

A Rust client library for the [Google OAuth2 API v2](https://developers.google.com/identity/protocols/oauth2), featuring a clean, modular architecture with shared HTTP request handling.

## Features

- **Modular Architecture** - Separated concerns: HTTP execution, API calls, configuration, and models
- **RequestExecutor Pattern** - Shared HTTP logic eliminates code duplication across API calls
- **Flattened API Surface** - Simple, intuitive method chaining without deep nesting
- **Type-Safe Builders** - Compile-time guarantees for API parameter construction
- **Zero-Cost Abstractions** - Efficient `Arc` sharing with minimal allocations

## Quick Start

```rust
use google_oauth2_v2::Service;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create service with default configuration
    let client = Client::new();
    let service = Service::new(client);

    // Get token information
    let tokeninfo = service
        .tokeninfo()
        .access_token("your_access_token".to_string())
        .do_call()
        .await?;

    tracing::info!("Token email: {:?}", tokeninfo.email);
    Ok(())
}
```

## API Examples

### Tokeninfo
```rust
let tokeninfo = service
    .tokeninfo()
    .access_token("ya29.a0ARrdaM...".to_string())
    .do_call()
    .await?;
```

### Userinfo
```rust
let userinfo = service
    .userinfo()
    .fields("email,name,picture".to_string())
    .do_call()
    .await?;
```

### Userinfo v2 Me
```rust
let userinfo = service
    .userinfo_v2_me()
    .do_call()
    .await?;
```

## Configuration

```rust
let service = Service::new(client)
    .with_base_path("https://custom.googleapis.com/".to_string())
    .with_user_agent("my-app/1.0".to_string());
```

## Architecture

The library follows a clean separation of concerns:

```
google-oauth2-v2/
├── config.rs      # API constants (BASE_PATH, SCOPES)
├── executor.rs    # RequestExecutor - shared HTTP logic
├── calls.rs       # API call builders (TokeninfoCall, etc.)
├── service.rs     # Main Service with flattened API
├── models.rs      # Data types (Tokeninfo, Userinfo)
└── error.rs       # Error types and handling
```

### RequestExecutor Pattern

All API calls delegate HTTP execution to a shared `RequestExecutor`:

```rust
// Before: Each call duplicated this logic
let response = client.get(url).header("User-Agent", agent).send().await?;
let status = response.status();
if status.is_success() { ... }

// After: Single line delegation
self.state.executor.get(url.as_str()).await
```

Benefits:
- **No code duplication** - HTTP logic in one place
- **Consistent error handling** - All calls use same error mapping
- **Easy to extend** - Add middleware, logging, or retry logic in one location

## Modules

### `config`
API constants and configuration values:
- `API_ID`, `API_NAME`, `API_VERSION`
- `BASE_PATH`, `BASE_PATH_TEMPLATE`
- OAuth2 scopes: `USERINFO_EMAIL_SCOPE`, `USERINFO_PROFILE_SCOPE`, `OPENID_SCOPE`

### `executor`
HTTP request execution:
- `RequestExecutor` - Handles GET/POST requests
- Shared response parsing and error handling
- `client()` accessor for builder methods

### `calls`
API call builders:
- `TokeninfoCall` - Query token metadata
- `UserinfoGetCall` - Get user profile info
- `UserinfoV2MeGetCall` - v2/me endpoint
- `ServiceState` - Shared state for all calls

### `service`
Main entry point:
- `Service::new()` - Create with default config
- `with_base_path()` - Custom API endpoint
- `with_user_agent()` - Custom user agent
- Direct access to all API calls

### `models`
Data structures:
- `Tokeninfo` - Token metadata (email, expires_in, scope, etc.)
- `Userinfo` - User profile (id, email, name, picture, etc.)

### `error`
Error handling:
- `OAuth2Error` - API errors, network errors, parsing errors
- `Result<T>` - Type alias for convenient error handling

## Safety

This crate follows Rust safety best practices:
- No `unsafe` code
- No `unwrap()` or `expect()` in production paths
- Proper error propagation with `?` operator
- Thread-safe `Arc` sharing for concurrent use

## License

BSD-3-Clause

## Contributing

This is a safety-critical systems codebase. All contributions must:
- Follow the existing modular architecture
- Maintain zero `unwrap()`/`expect()` usage
- Include proper error handling
- Pass clippy lints (`cargo clippy -- -D warnings`)
