#[cfg(test)]
mod tests {
    use mockall::mock;
    use async_trait::async_trait;
    use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};
    use crate::domain::entities::auth::{LoginRequest, SignupRequest};
    use crate::domain::entities::user::User;
    use crate::application::errors::auth_error::AuthError;
    use crate::domain::services::auth_service::AuthService;

    mock! {
        pub AuthService {}
        #[async_trait]
        impl AuthService for AuthService {
            async fn guest_login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
            async fn signup(&self, req: &SignupRequest) -> Result<(), AuthError>;
            async fn login(&self, req: &LoginRequest) -> Result<Option<User>, AuthError>;
        }
    }

    // サインアップ　成功
    #[actix_rt::test]
    async fn test_signup_success() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| Ok(()));

        let req = SignupRequest {
            name: "test_user2".to_string(),
            email: "123@gmail.com".to_string(),
            password: "123".to_string()
        };

        let result = mock_service.signup(&req).await;

        assert!(result.is_ok());
    }

    // サインアップ　失敗　メールアドレスに @ が含まれていない
    #[actix_rt::test]
    async fn test_signup_without_at_sign_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("email", ValidationError::new("email_missing_at_sign"));

                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user2".to_string(),
            email: "1-2_3@gmail.com".to_string(),
            password: "Valid1234567".to_string()
        };
        let result = mock_service.signup(&req).await;

        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("email") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "email_missing_at_sign");
                            assert!(has_error, "Expected validation error with code 'email_missing_at_sign'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"email_missing_at_sign"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'email' field");
                }
            }
            _ => panic!("Expected a validation error with message 'email_missing_at_sign'"),
        }
    }

    // サインアップ　失敗　特殊文字が含まれていない　['.', '_', '-']
    #[actix_rt::test]
    async fn test_signup_email_missing_special_character_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("email", ValidationError::new("email_missing_special_character_in_domain"));

                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user".to_string(),
            email: "userexample@gmail.com".to_string(),
            password: "Valid123".to_string(),
        };

        let result = mock_service.signup(&req).await;

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("email") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "email_missing_special_character_in_domain");
                            assert!(has_error, "Expected validation error with code 'email_missing_special_character_in_domain'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"email_missing_special_character_in_domain"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'email' field");
                }
            }
            _ => panic!("Expected a validation error with message 'email_missing_special_character_in_domain'"),
        }
    }

    // サインアップ　失敗　無効な特殊文字が含まれている
    #[actix_rt::test]
    async fn test_signup_with_invalid_special_char_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("email", ValidationError::new("email_contains_invalid_special_characters"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user".to_string(),
            email: "u]s=|er@example.com".to_string(),
            password: "Short342335".to_string(),
        };

        let result = mock_service.signup(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("email") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "email_contains_invalid_special_characters");
                            assert!(has_error, "Expected validation error with code 'email_contains_invalid_special_characters'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"email_contains_invalid_special_characters"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'email' field");
                }
            }
            _ => panic!("Expected a validation error with code 'email_contains_invalid_special_characters'"),
        }
    }

    // サインアップ　失敗　ドメイン名に . が含まれていない
    #[actix_rt::test]
    async fn test_signup_invalid_domain_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("email", ValidationError::new("email_invalid_domain"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user".to_string(),
            email: "user@gmailcom".to_string(),
            password: "Valid4523".to_string(),
        };

        let result = mock_service.signup(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("email") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "email_invalid_domain");
                            assert!(has_error, "Expected validation error with code 'email_invalid_domain'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"email_invalid_domain"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'email' field");
                }
            }
            _ => panic!("Expected a validation error with code 'email_invalid_domain'"),
        }
    }

    // サインアップ　失敗　パスワードが短い
    #[actix_rt::test]
    async fn test_signup_password_too_short_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("password", ValidationError::new("password_too_short"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user".to_string(),
            email: "user@example.com".to_string(),
            password: "short".to_string(),
        };

        let result = mock_service.signup(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("password") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "password_too_short");
                            assert!(has_error, "Expected validation error with code 'password_too_short'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"password_too_short"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'password' field");
                }
            }
            _ => panic!("Expected a validation error with code 'password_too_short'"),
        }
    }

    // サインアップ　失敗　パスワードに数字が含まれていない
    #[actix_rt::test]
    async fn test_signup_password_missing_digit_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("password", ValidationError::new("password_no_digit"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user".to_string(),
            email: "user@example.com".to_string(),
            password: "short".to_string(),
        };

        let result = mock_service.signup(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("password") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "password_no_digit");
                            assert!(has_error, "Expected validation error with code 'password_no_digit'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"password_no_digit"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'password' field");
                }
            }
            _ => panic!("Expected a validation error with code 'password_no_digit'"),
        }
    }

    // サインアップ　失敗　パスワードに大文字が含まれていない
    #[actix_rt::test]
    async fn test_signup_password_missing_uppercase_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_signup()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("password", ValidationError::new("password_no_uppercase"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = SignupRequest {
            name: "test_user".to_string(),
            email: "user@example.com".to_string(),
            password: "short".to_string(),
        };

        let result = mock_service.signup(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("password") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "password_no_uppercase");
                            assert!(has_error, "Expected validation error with code 'password_no_uppercase'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"password_no_uppercase"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'password' field");
                }
            }
            _ => panic!("Expected a validation error with code 'password_no_uppercase'"),
        }
    }

    // ログイン　成功
    #[actix_rt::test]
    async fn test_login_success() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_login()
            .returning(|_| Ok(Some(User {
                id: 1,
                name: "test_user1".to_string(),
                email: "test@gmail.com".to_string(),
                token: "test_token".to_string(),
            })));

        let req = LoginRequest {
            name: "test_user1".to_string(),
            email: "test@gmail.com".to_string(),
            password: "Password123".to_string()
        };

        let result = mock_service.login(&req).await;

        assert!(result.is_ok());
        let user_data = result.unwrap();
        assert!(user_data.is_some());
        let user = user_data.unwrap();
        assert_eq!(user.name, "test_user1");
        assert_eq!(user.token, "test_token");
    }

    // ログイン　失敗　パスワードが短い
    #[actix_rt::test]
    async fn test_login_password_too_short_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_login()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("password", ValidationError::new("password_too_short"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = LoginRequest {
            name: "test_user1".to_string(),
            email: "test@gmail.com".to_string(),
            password: "P123".to_string()
        };

        let result = mock_service.login(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("password") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "password_too_short");
                            assert!(has_error, "Expected validation error with code 'password_too_short'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"password_too_short"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'password' field");
                }
            }
            _ => panic!("Expected a validation error with code 'password_too_short'"),
        }
    }

    // ログイン　失敗　パスワードに数字が含まれていない
    #[actix_rt::test]
    async fn test_login_password_missing_digit_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_login()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("password", ValidationError::new("password_no_digit"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = LoginRequest {
            name: "test_user1".to_string(),
            email: "test@gmail.com".to_string(),
            password: "Password".to_string()
        };

        let result = mock_service.login(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("password") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "password_no_digit");
                            assert!(has_error, "Expected validation error with code 'password_no_digit'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"password_no_digit"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'password' field");
                }
            }
            _ => panic!("Expected a validation error with code 'password_no_digit'"),
        }
    }

    // ログイン　失敗　パスワードに大文字が含まれていない
    #[actix_rt::test]
    async fn test_login_password_missing_uppercase_failure() {
        let mut mock_service = MockAuthService::new();

        mock_service
            .expect_login()
            .returning(|_| {
                let mut validation_errors = ValidationErrors::new();
                validation_errors.add("password", ValidationError::new("password_no_uppercase"));
                
                Err(AuthError::ValidationError(validation_errors))
            });

        let req = LoginRequest {
            name: "test_user1".to_string(),
            email: "test@gmail.com".to_string(),
            password: "password123".to_string()
        };

        let result = mock_service.login(&req).await;
        dbg!(&result);

        match result {
            Err(AuthError::ValidationError(validation_errors)) => {
                if let Some(errors) = validation_errors.0.get("password") {
                    match errors {
                        ValidationErrorsKind::Field(errors) => {
                            let has_error = errors.iter().any(|e| e.code == "password_no_uppercase");
                            assert!(has_error, "Expected validation error with code 'password_no_uppercase'");

                            let error_codes: Vec<&str> = errors.iter().map(|e| e.code.as_ref()).collect();
                            assert!(error_codes.contains(&"password_no_uppercase"));
                        }
                        _ => panic!("Expected ValidationErrorsKind::Field"),
                    }
                } else {
                    panic!("Expected validation error for 'password' field");
                }
            }
            _ => panic!("Expected a validation error with code 'password_no_uppercase'"),
        }
    }
}
