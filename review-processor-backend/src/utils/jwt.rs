use std::env;
use google_oauth::AsyncClient;

pub async fn verify_jwt(token: &str) -> Result<String, String> {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = AsyncClient::new(client_id);
    let decoded = client.validate_id_token(&token)
        .await.map_err(|_| "Failed to validate token")?;

    Ok(decoded.email.unwrap_or_default())
}
