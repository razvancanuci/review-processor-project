use std::env;
use google_oauth::AsyncClient;

/// Verifies a Google OAuth JWT token and extracts the user's email
///
/// This function validates a JWT token using Google's OAuth service
/// and returns the email address associated with the token if valid.
///
/// # Arguments
///
/// * `token` - The JWT token to verify
///
/// # Returns
///
/// Returns a `Result` containing:
/// * `Ok(String)` - The email address extracted from the token
/// * `Err(String)` - An error message if token validation fails
///
/// # Environment Variables
///
/// * `GOOGLE_CLIENT_ID` - The Google OAuth client ID used for token validation
///
/// # Panics
///
/// This function will panic if the `GOOGLE_CLIENT_ID` environment variable is not set
///
/// # Examples
///
/// ```rust
/// use review_processor_backend::utils::jwt;
///
/// #[tokio::main]
/// async fn main() {
///     let token = "your.jwt.token";
///     match jwt::verify_jwt(token).await {
///         Ok(email) => println!("Verified email: {}", email),
///         Err(e) => println!("Verification failed: {}", e),
///     }
/// }
/// ```
pub async fn verify_jwt(token: &str) -> Result<String, String> {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = AsyncClient::new(client_id);
    let decoded = client.validate_id_token(&token)
        .await.map_err(|_| "Failed to validate token")?;

    Ok(decoded.email.unwrap_or_default())
}
