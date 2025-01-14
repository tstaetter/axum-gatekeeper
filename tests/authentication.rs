use axum_gatekeeper::tokens::{Claims, Token};

mod init_env;

#[cfg(feature = "authentication")]
use axum_gatekeeper::tokens::AuthenticationToken;
#[cfg(feature = "authentication")]
use axum_gatekeeper::tokens::RefreshToken;

#[test]
fn test_can_create_auth_token() -> anyhow::Result<()> {
    init_env::init_test_env();

    let uuid = uuid::Uuid::new_v4();
    let claims = Claims {
        exp: 0,
        iat: 0,
        sub: uuid.to_string(),
    };
    let token = AuthenticationToken::encode(&claims, "secret".to_string())?;
    let token = AuthenticationToken::new(token, claims);

    assert_eq!(token.get_claims().sub, uuid.to_string());

    Ok(())
}

#[test]
fn test_can_create_refresh_token() -> anyhow::Result<()> {
    init_env::init_test_env();

    let uuid = uuid::Uuid::new_v4();
    let claims = Claims {
        exp: 0,
        iat: 0,
        sub: uuid.to_string(),
    };
    let token = RefreshToken::encode(&claims, "secret".to_string())?;
    let token = RefreshToken::new(token, claims);

    assert_eq!(token.get_claims().sub, uuid.to_string());

    Ok(())
}
