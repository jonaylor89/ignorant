use crate::CheckResult;
use reqwest::Client;
use serde_json::Value;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use url::form_urlencoded;

type HmacSha256 = Hmac<Sha256>;

const USERS_LOOKUP_URL: &str = "https://i.instagram.com/api/v1/users/lookup/";
const SIG_KEY_VERSION: &str = "4";
const IG_SIG_KEY: &str = "e6358aeede676184b9fe702b30f4fd35e71744605e39d2181a34cede076b3c33";

fn generate_signature(data: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(IG_SIG_KEY.as_bytes()).unwrap();
    mac.update(data.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());
    
    format!("ig_sig_key_version={}&signed_body={}.{}", 
            SIG_KEY_VERSION, 
            signature, 
            form_urlencoded::byte_serialize(data.as_bytes()).collect::<String>())
}

fn generate_data(phone_number_raw: &str) -> String {
    let mut data = HashMap::new();
    data.insert("login_attempt_count", "0");
    data.insert("directly_sign_in", "true");
    data.insert("source", "default");
    data.insert("q", phone_number_raw);
    data.insert("ig_sig_key_version", SIG_KEY_VERSION);
    
    serde_json::to_string(&data).unwrap()
}

const INSTAGRAM_USER_AGENT: &str = "Instagram 101.0.0.15.120";
const NO_USERS_FOUND_MSG: &str = "No users found";

pub async fn check_instagram(phone: &str, country_code: &str, client: &Client) -> CheckResult {
    let phone_number = format!("{country_code}{phone}");
    let data = generate_signature(&generate_data(&phone_number));
    
    let headers = [
        ("Accept-Language", "en-US"),
        ("User-Agent", INSTAGRAM_USER_AGENT),
        ("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8"),
        ("Accept-Encoding", "gzip, deflate"),
        ("X-FB-HTTP-Engine", "Liger"),
        ("Connection", "close"),
    ];
    
    let mut request = client.post(USERS_LOOKUP_URL);
    for (key, value) in &headers {
        request = request.header(*key, *value);
    }
    
    match request.body(data).send().await {
        Ok(response) => {
            match response.json::<Value>().await {
                Ok(json) => {
                    let exists = json.get("message")
                        .and_then(|msg| msg.as_str()) != Some(NO_USERS_FOUND_MSG);
                    
                    CheckResult::new("instagram", "instagram.com", "other").with_exists(exists)
                }
                Err(_) => CheckResult::new("instagram", "instagram.com", "other").with_rate_limit()
            }
        }
        Err(_) => CheckResult::new("instagram", "instagram.com", "other").with_rate_limit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_signature() {
        let test_data = r#"{"login_attempt_count":"0","directly_sign_in":"true","source":"default","q":"33123456789","ig_sig_key_version":"4"}"#;
        let signature = generate_signature(test_data);
        
        assert!(signature.starts_with("ig_sig_key_version=4&signed_body="));
        assert!(signature.contains(&form_urlencoded::byte_serialize(test_data.as_bytes()).collect::<String>()));
    }

    #[test]
    fn test_generate_data() {
        let phone_number = "33123456789";
        let data = generate_data(phone_number);
        let parsed: serde_json::Value = serde_json::from_str(&data).unwrap();
        
        assert_eq!(parsed["q"], phone_number);
        assert_eq!(parsed["login_attempt_count"], "0");
        assert_eq!(parsed["directly_sign_in"], "true");
        assert_eq!(parsed["source"], "default");
        assert_eq!(parsed["ig_sig_key_version"], "4");
    }

    #[tokio::test]
    async fn test_check_instagram_no_users_found() {
        // Note: This test checks the logic but doesn't use mock server
        // since the function uses hardcoded URLs. In a real implementation,
        // we'd want to make URLs configurable for testing.
        
        // Test the logic directly
        let phone = "123456789";
        let country_code = "33";
        let phone_number = format!("{country_code}{phone}");
        let data = generate_data(&phone_number);
        let parsed: serde_json::Value = serde_json::from_str(&data).unwrap();
        
        assert_eq!(parsed["q"], phone_number);
        
        // Test the response parsing logic
        let json_response = json!({"message": "No users found"});
        let exists = json_response.get("message")
            .and_then(|msg| msg.as_str()) != Some("No users found");
        assert!(!exists);
    }

    #[test]
    fn test_response_parsing_user_exists() {
        // Test the response parsing logic for user exists
        let json_response = json!({"users": [{"username": "test"}]});
        let exists = json_response.get("message")
            .and_then(|msg| msg.as_str()) != Some("No users found");
        assert!(exists);
    }

    #[test]
    fn test_response_parsing_empty_response() {
        // Test the response parsing logic for empty response
        let json_response = json!({});
        let exists = json_response.get("message")
            .and_then(|msg| msg.as_str()) != Some("No users found");
        assert!(exists);
    }
}