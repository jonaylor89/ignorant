//! Amazon account checking module
//! 
//! This module checks if a phone number is associated with an Amazon account
//! by attempting to sign in and analyzing the response for password prompts.

use crate::{CheckResult, user_agents::USER_AGENTS};
use reqwest::Client;
use std::collections::HashMap;
use rand::Rng;

/// Extract hidden form fields from HTML content
fn extract_form_data(html_content: &str) -> HashMap<String, String> {
    let mut form_data = HashMap::new();
    
    // Extract CSRF tokens and other hidden inputs using simple string parsing
    let lines: Vec<&str> = html_content.lines().collect();
    for line in lines {
        if line.contains("<input") && line.contains("type=\"hidden\"") {
            if let (Some(name_start), Some(value_start)) = (
                line.find("name=\"").map(|i| i + 6),
                line.find("value=\"").map(|i| i + 7)
            ) {
                if let (Some(name_end), Some(value_end)) = (
                    line[name_start..].find('\"').map(|i| i + name_start),
                    line[value_start..].find('\"').map(|i| i + value_start)
                ) {
                    let name = &line[name_start..name_end];
                    let value = &line[value_start..value_end];
                    form_data.insert(name.to_string(), value.to_string());
                }
            }
        }
    }
    form_data
}

const AMAZON_SIGNIN_URL: &str = "https://www.amazon.com/ap/signin?openid.pape.max_auth_age=0&openid.return_to=https%3A%2F%2Fwww.amazon.com%2F%3F_encoding%3DUTF8%26ref_%3Dnav_ya_signin&openid.identity=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.assoc_handle=usflex&openid.mode=checkid_setup&openid.claimed_id=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0%2Fidentifier_select&openid.ns=http%3A%2F%2Fspecs.openid.net%2Fauth%2F2.0&";
const AMAZON_SUBMIT_URL: &str = "https://www.amazon.com/ap/signin/";
const PASSWORD_MISSING_ALERT: &str = "auth-password-missing-alert";

/// Check if a phone number is associated with an Amazon account
/// 
/// This function attempts to sign in to Amazon using the phone number as an email.
/// If Amazon prompts for a password, it indicates the phone number is associated with an account.
/// 
/// # Arguments
/// 
/// * `phone` - The phone number to check (without country code)
/// * `country_code` - The country code for the phone number
/// * `client` - HTTP client for making requests
/// 
/// # Returns
/// 
/// A `CheckResult` indicating whether the phone number exists on Amazon
pub async fn check_amazon(phone: &str, country_code: &str, client: &Client) -> CheckResult {
    let user_agent = USER_AGENTS.chrome[rand::thread_rng().gen_range(0..USER_AGENTS.chrome.len())];
    
    match client.get(AMAZON_SIGNIN_URL).header("User-Agent", user_agent).send().await {
        Ok(response) => {
            match response.text().await {
                Ok(html_content) => {
                    let mut form_data = extract_form_data(&html_content);
                    
                    // Set email field
                    form_data.insert("email".to_owned(), format!("{country_code}{phone}"));
                    
                    // Submit form
                    match client.post(AMAZON_SUBMIT_URL)
                        .form(&form_data)
                        .send()
                        .await {
                        Ok(response) => {
                            match response.text().await {
                                Ok(html) => {
                                    let exists = html.contains(PASSWORD_MISSING_ALERT);
                                    CheckResult::new("amazon", "amazon.com", "login").with_exists(exists)
                                }
                                Err(_) => CheckResult::new("amazon", "amazon.com", "login").with_rate_limit()
                            }
                        }
                        Err(_) => CheckResult::new("amazon", "amazon.com", "login").with_rate_limit()
                    }
                }
                Err(_) => CheckResult::new("amazon", "amazon.com", "login").with_rate_limit()
            }
        }
        Err(_) => CheckResult::new("amazon", "amazon.com", "login").with_rate_limit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_form_data() {
        let html = r#"
            <html>
                <form>
                    <input type="hidden" name="csrf_token" value="abc123">
                    <input type="hidden" name="session_id" value="def456">
                    <input type="text" name="email" value="">
                </form>
            </html>
        "#;
        
        let form_data = extract_form_data(html);
        
        assert_eq!(form_data.get("csrf_token"), Some(&"abc123".to_string()));
        assert_eq!(form_data.get("session_id"), Some(&"def456".to_string()));
        // Should not extract non-hidden inputs
        assert!(!form_data.contains_key("email"));
    }

    #[test]
    fn test_extract_form_data_malformed() {
        let html = r#"
            <input type="hidden" name="token" value="test
            <input type="hidden" name="incomplete
        "#;
        
        let form_data = extract_form_data(html);
        
        // Should handle malformed HTML gracefully
        assert!(form_data.is_empty() || form_data.len() <= 1);
    }

    #[test]
    fn test_password_detection_logic() {
        // Test HTML parsing for password alert
        let html_with_alert = r#"<div id="auth-password-missing-alert">Password required</div>"#;
        let html_without_alert = r#"<div>No account found</div>"#;
        
        assert!(html_with_alert.contains(PASSWORD_MISSING_ALERT));
        assert!(!html_without_alert.contains(PASSWORD_MISSING_ALERT));
    }

    #[tokio::test]
    async fn test_check_amazon_network_error() {
        // Use a client with an invalid URL to simulate network failure
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(1))
            .build()
            .unwrap();
        
        let result = check_amazon("123456789", "33", &client).await;

        assert_eq!(result.name, "amazon");
        assert!(result.rate_limit);
        assert!(!result.exists);
    }
}