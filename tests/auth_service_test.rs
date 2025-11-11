use template_rust_backend::enums::UserRole;
use template_rust_backend::services::auth_service::AuthService;
use uuid::Uuid;

#[test]
fn test_hash_password() {
    let password = "test_password_123";
    let hash = AuthService::hash_password(password).unwrap();
    assert!(!hash.is_empty());
    assert_ne!(hash, password);
}

#[test]
fn test_verify_password_correct() {
    let password = "test_password_123";
    let hash = AuthService::hash_password(password).unwrap();
    let result = AuthService::verify_password(password, &hash).unwrap();
    assert!(result);
}

#[test]
fn test_verify_password_incorrect() {
    let password = "test_password_123";
    let hash = AuthService::hash_password(password).unwrap();
    let result = AuthService::verify_password("wrong_password", &hash).unwrap();
    assert!(!result);
}

#[test]
fn test_generate_and_verify_token() {
    let secret = "test_secret_key_for_jwt_token_generation";
    let user_id = Uuid::now_v7();
    let tenant_id = Uuid::now_v7();
    let email = "test@example.com".to_string();
    let role = UserRole::Admin;
    let expiration_minutes = 60;

    let token = AuthService::generate_token(
        user_id,
        tenant_id,
        email.clone(),
        role,
        secret,
        expiration_minutes,
    )
    .unwrap();

    assert!(!token.is_empty());

    let claims = AuthService::verify_token(&token, secret).unwrap();
    assert_eq!(claims.user_id, user_id);
    assert_eq!(claims.tenant_id, tenant_id);
    assert_eq!(claims.email, email);
    assert_eq!(claims.role, role);
}

#[test]
fn test_verify_token_invalid() {
    let secret = "test_secret_key";
    let invalid_token = "invalid.token.here";
    let result = AuthService::verify_token(invalid_token, secret);
    assert!(result.is_err());
}

#[test]
fn test_verify_token_wrong_secret() {
    let secret1 = "secret_key_1";
    let secret2 = "secret_key_2";
    let user_id = Uuid::now_v7();
    let tenant_id = Uuid::now_v7();

    let token = AuthService::generate_token(
        user_id,
        tenant_id,
        "test@example.com".to_string(),
        UserRole::Admin,
        secret1,
        60,
    )
    .unwrap();

    let result = AuthService::verify_token(&token, secret2);
    assert!(result.is_err());
}
