use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tokeninfo {
    pub audience: Option<String>,
    pub email: Option<String>,
    pub expires_in: Option<u64>,
    pub issued_to: Option<String>,
    pub scope: Option<String>,
    pub user_id: Option<String>,
    pub verified_email: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Userinfo {
    pub id: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
    pub hd: Option<String>,
    pub link: Option<String>,
    pub gender: Option<String>,
    pub verified_email: Option<bool>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // -------------------------
    // Tokeninfo tests
    // -------------------------

    #[test]
    fn tokeninfo_deserializes_correctly() {
        let json = r#"{
            "audience": "test_audience",
            "email": "test@example.com",
            "expires_in": 3600,
            "issued_to": "test_client",
            "scope": "email profile",
            "user_id": "12345",
            "verified_email": true
        }"#;

        let token: Tokeninfo = serde_json::from_str(json).unwrap();

        assert_eq!(token.audience, Some("test_audience".to_string()));
        assert_eq!(token.email, Some("test@example.com".to_string()));
        assert_eq!(token.expires_in, Some(3600));
        assert_eq!(token.issued_to, Some("test_client".to_string()));
        assert_eq!(token.scope, Some("email profile".to_string()));
        assert_eq!(token.user_id, Some("12345".to_string()));
        assert_eq!(token.verified_email, Some(true));
    }

    #[test]
    fn tokeninfo_handles_empty_json() {
        let json = r#"{}"#;

        let token: Tokeninfo = serde_json::from_str(json).unwrap();

        assert!(token.audience.is_none());
        assert!(token.email.is_none());
        assert!(token.expires_in.is_none());
        assert!(token.issued_to.is_none());
        assert!(token.scope.is_none());
        assert!(token.user_id.is_none());
        assert!(token.verified_email.is_none());
    }

    #[test]
    fn tokeninfo_roundtrip_serialization() {
        let token = Tokeninfo {
            audience: Some("aud".to_string()),
            email: Some("a@b.com".to_string()),
            expires_in: Some(1234),
            issued_to: Some("client".to_string()),
            scope: Some("email".to_string()),
            user_id: Some("42".to_string()),
            verified_email: Some(false),
        };

        let json = serde_json::to_string(&token).unwrap();
        let decoded: Tokeninfo = serde_json::from_str(&json).unwrap();

        assert_eq!(token.email, decoded.email);
        assert_eq!(token.expires_in, decoded.expires_in);
    }

    // -------------------------
    // Userinfo tests
    // -------------------------

    #[test]
    fn userinfo_deserializes_correctly() {
        let json = r#"{
            "id": "12345",
            "email": "test@example.com",
            "name": "John Doe",
            "given_name": "John",
            "family_name": "Doe",
            "picture": "https://example.com/pic.jpg",
            "locale": "en",
            "hd": "example.com",
            "link": "https://example.com",
            "gender": "male",
            "verified_email": true
        }"#;

        let user: Userinfo = serde_json::from_str(json).unwrap();

        assert_eq!(user.id, Some("12345".to_string()));
        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert_eq!(user.name, Some("John Doe".to_string()));
        assert_eq!(user.given_name, Some("John".to_string()));
        assert_eq!(user.family_name, Some("Doe".to_string()));
        assert_eq!(user.picture, Some("https://example.com/pic.jpg".to_string()));
        assert_eq!(user.locale, Some("en".to_string()));
        assert_eq!(user.hd, Some("example.com".to_string()));
        assert_eq!(user.link, Some("https://example.com".to_string()));
        assert_eq!(user.gender, Some("male".to_string()));
        assert_eq!(user.verified_email, Some(true));
    }

    #[test]
    fn userinfo_partial_json_works() {
        let json = r#"{
            "email": "test@example.com",
            "name": "John Doe"
        }"#;

        let user: Userinfo = serde_json::from_str(json).unwrap();

        assert_eq!(user.email, Some("test@example.com".to_string()));
        assert_eq!(user.name, Some("John Doe".to_string()));

        // all others must be None
        assert!(user.id.is_none());
        assert!(user.given_name.is_none());
        assert!(user.family_name.is_none());
    }

    #[test]
    fn userinfo_empty_json() {
        let json = r#"{}"#;

        let user: Userinfo = serde_json::from_str(json).unwrap();

        assert!(user.email.is_none());
        assert!(user.id.is_none());
        assert!(user.name.is_none());
        assert!(user.verified_email.is_none());
    }

    #[test]
    fn userinfo_roundtrip_serialization() {
        let user = Userinfo {
            id: Some("1".to_string()),
            email: Some("a@b.com".to_string()),
            name: Some("Test User".to_string()),
            given_name: Some("Test".to_string()),
            family_name: Some("User".to_string()),
            picture: None,
            locale: None,
            hd: None,
            link: None,
            gender: None,
            verified_email: Some(true),
        };

        let json = serde_json::to_string(&user).unwrap();
        let decoded: Userinfo = serde_json::from_str(&json).unwrap();

        assert_eq!(user.email, decoded.email);
        assert_eq!(user.name, decoded.name);
        assert_eq!(user.verified_email, decoded.verified_email);
    }
}
