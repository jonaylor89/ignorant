use crate::{CheckResult, user_agents::USER_AGENTS};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use rand::Rng;

const SNAPCHAT_BASE_URL: &str = "https://accounts.snapchat.com";
const SNAPCHAT_VALIDATE_URL: &str = "https://accounts.snapchat.com/accounts/validate_phone_number";
const XSRF_TOKEN_COOKIE: &str = "xsrf_token";
const TAKEN_NUMBER_STATUS: &str = "TAKEN_NUMBER";
const OK_STATUS: &str = "OK";

pub async fn check_snapchat(phone: &str, country_code: &str, client: &Client) -> CheckResult {
    let convert_to_country_code = get_country_code_map();
    let user_agent = USER_AGENTS.chrome[rand::thread_rng().gen_range(0..USER_AGENTS.chrome.len())];
    
    let headers = [
        ("User-Agent", user_agent),
        ("Accept", "*/*"),
        ("Accept-Language", "en,en-US;q=0.5"),
        ("Content-Type", "application/x-www-form-urlencoded; charset=utf-8"),
        ("Origin", SNAPCHAT_BASE_URL),
        ("DNT", "1"),
        ("Connection", "keep-alive"),
        ("Sec-GPC", "1"),
        ("TE", "Trailers"),
    ];
    
    // First, get the main page to obtain xsrf_token
    let mut request = client.get(SNAPCHAT_BASE_URL);
    for (key, value) in &headers {
        request = request.header(*key, *value);
    }
    
    match request.send().await {
        Ok(response) => {
            let mut cookies = response.cookies();
            let xsrf_token = cookies
                .find(|cookie| cookie.name() == XSRF_TOKEN_COOKIE)
                .map(|cookie| cookie.value().to_owned());
            
            if let Some(xsrf_token) = xsrf_token {
                if let Some(&country_code_str) = convert_to_country_code.get(country_code) {
                    let mut form_data = HashMap::new();
                    form_data.insert("phone_country_code", country_code_str);
                    form_data.insert("phone_number", phone);
                    form_data.insert("xsrf_token", &xsrf_token);
                    
                    let mut request = client.post(SNAPCHAT_VALIDATE_URL);
                    for (key, value) in &headers {
                        request = request.header(*key, *value);
                    }
                    
                    match request.form(&form_data).send().await {
                        Ok(response) => {
                            match response.json::<Value>().await {
                                Ok(json) => {
                                    let status = json.get("status_code").and_then(|s| s.as_str());
                                    
                                    match status {
                                        Some(TAKEN_NUMBER_STATUS) => {
                                            CheckResult::new("snapchat", "snapchat.com", "register").with_exists(true)
                                        }
                                        Some(OK_STATUS) => {
                                            CheckResult::new("snapchat", "snapchat.com", "register").with_exists(false)
                                        }
                                        _ => {
                                            CheckResult::new("snapchat", "snapchat.com", "register").with_rate_limit()
                                        }
                                    }
                                }
                                Err(_) => CheckResult::new("snapchat", "snapchat.com", "register").with_rate_limit()
                            }
                        }
                        Err(_) => CheckResult::new("snapchat", "snapchat.com", "register").with_rate_limit()
                    }
                } else {
                    CheckResult::new("snapchat", "snapchat.com", "register").with_rate_limit()
                }
            } else {
                CheckResult::new("snapchat", "snapchat.com", "register").with_rate_limit()
            }
        }
        Err(_) => CheckResult::new("snapchat", "snapchat.com", "register").with_rate_limit()
    }
}

fn get_country_code_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("1", "VI");
    map.insert("49", "DE");
    map.insert("33", "FR");
    map.insert("44", "JE");
    map.insert("247", "AC");
    map.insert("376", "AD");
    map.insert("971", "AE");
    map.insert("93", "AF");
    map.insert("355", "AL");
    map.insert("374", "AM");
    map.insert("244", "AO");
    map.insert("54", "AR");
    map.insert("43", "AT");
    map.insert("61", "CX");
    map.insert("297", "AW");
    map.insert("358", "FI");
    map.insert("994", "AZ");
    map.insert("387", "BA");
    map.insert("880", "BD");
    map.insert("32", "BE");
    map.insert("226", "BF");
    map.insert("359", "BG");
    map.insert("973", "BH");
    map.insert("257", "BI");
    map.insert("229", "BJ");
    map.insert("590", "MF");
    map.insert("673", "BN");
    map.insert("591", "BO");
    map.insert("599", "CW");
    map.insert("55", "BR");
    map.insert("975", "BT");
    map.insert("267", "BW");
    map.insert("375", "BY");
    map.insert("501", "BZ");
    map.insert("243", "CD");
    map.insert("236", "CF");
    map.insert("242", "CG");
    map.insert("41", "CH");
    map.insert("225", "CI");
    map.insert("682", "CK");
    map.insert("56", "CL");
    map.insert("237", "CM");
    map.insert("86", "CN");
    map.insert("57", "CO");
    map.insert("506", "CR");
    map.insert("53", "CU");
    map.insert("238", "CV");
    map.insert("357", "CY");
    map.insert("420", "CZ");
    map.insert("253", "DJ");
    map.insert("45", "DK");
    map.insert("213", "DZ");
    map.insert("593", "EC");
    map.insert("372", "EE");
    map.insert("20", "EG");
    map.insert("212", "MA");
    map.insert("291", "ER");
    map.insert("34", "ES");
    map.insert("251", "ET");
    map.insert("679", "FJ");
    map.insert("500", "FK");
    map.insert("691", "FM");
    map.insert("298", "FO");
    map.insert("241", "GA");
    map.insert("995", "GE");
    map.insert("594", "GF");
    map.insert("233", "GH");
    map.insert("350", "GI");
    map.insert("299", "GL");
    map.insert("220", "GM");
    map.insert("224", "GN");
    map.insert("240", "GQ");
    map.insert("30", "GR");
    map.insert("502", "GT");
    map.insert("245", "GW");
    map.insert("592", "GY");
    map.insert("852", "HK");
    map.insert("504", "HN");
    map.insert("385", "HR");
    map.insert("509", "HT");
    map.insert("36", "HU");
    map.insert("62", "ID");
    map.insert("353", "IE");
    map.insert("972", "IL");
    map.insert("91", "IN");
    map.insert("246", "IO");
    map.insert("964", "IQ");
    map.insert("98", "IR");
    map.insert("354", "IS");
    map.insert("39", "VA");
    map.insert("962", "JO");
    map.insert("81", "JP");
    map.insert("254", "KE");
    map.insert("996", "KG");
    map.insert("855", "KH");
    map.insert("686", "KI");
    map.insert("269", "KM");
    map.insert("850", "KP");
    map.insert("82", "KR");
    map.insert("965", "KW");
    map.insert("7", "RU");
    map.insert("856", "LA");
    map.insert("961", "LB");
    map.insert("423", "LI");
    map.insert("94", "LK");
    map.insert("231", "LR");
    map.insert("266", "LS");
    map.insert("370", "LT");
    map.insert("352", "LU");
    map.insert("371", "LV");
    map.insert("218", "LY");
    map.insert("377", "MC");
    map.insert("373", "MD");
    map.insert("382", "ME");
    map.insert("261", "MG");
    map.insert("692", "MH");
    map.insert("389", "MK");
    map.insert("223", "ML");
    map.insert("95", "MM");
    map.insert("976", "MN");
    map.insert("853", "MO");
    map.insert("596", "MQ");
    map.insert("222", "MR");
    map.insert("356", "MT");
    map.insert("230", "MU");
    map.insert("960", "MV");
    map.insert("265", "MW");
    map.insert("52", "MX");
    map.insert("60", "MY");
    map.insert("258", "MZ");
    map.insert("264", "NA");
    map.insert("687", "NC");
    map.insert("227", "NE");
    map.insert("672", "NF");
    map.insert("234", "NG");
    map.insert("505", "NI");
    map.insert("31", "NL");
    map.insert("47", "SJ");
    map.insert("977", "NP");
    map.insert("674", "NR");
    map.insert("683", "NU");
    map.insert("64", "NZ");
    map.insert("968", "OM");
    map.insert("507", "PA");
    map.insert("51", "PE");
    map.insert("689", "PF");
    map.insert("675", "PG");
    map.insert("63", "PH");
    map.insert("92", "PK");
    map.insert("48", "PL");
    map.insert("508", "PM");
    map.insert("970", "PS");
    map.insert("351", "PT");
    map.insert("680", "PW");
    map.insert("595", "PY");
    map.insert("974", "QA");
    map.insert("262", "YT");
    map.insert("40", "RO");
    map.insert("381", "RS");
    map.insert("250", "RW");
    map.insert("966", "SA");
    map.insert("677", "SB");
    map.insert("248", "SC");
    map.insert("249", "SD");
    map.insert("46", "SE");
    map.insert("65", "SG");
    map.insert("290", "TA");
    map.insert("386", "SI");
    map.insert("421", "SK");
    map.insert("232", "SL");
    map.insert("378", "SM");
    map.insert("221", "SN");
    map.insert("252", "SO");
    map.insert("597", "SR");
    map.insert("211", "SS");
    map.insert("239", "ST");
    map.insert("503", "SV");
    map.insert("963", "SY");
    map.insert("268", "SZ");
    map.insert("235", "TD");
    map.insert("228", "TG");
    map.insert("66", "TH");
    map.insert("992", "TJ");
    map.insert("690", "TK");
    map.insert("670", "TL");
    map.insert("993", "TM");
    map.insert("216", "TN");
    map.insert("676", "TO");
    map.insert("90", "TR");
    map.insert("688", "TV");
    map.insert("886", "TW");
    map.insert("255", "TZ");
    map.insert("380", "UA");
    map.insert("256", "UG");
    map.insert("598", "UY");
    map.insert("998", "UZ");
    map.insert("58", "VE");
    map.insert("84", "VN");
    map.insert("678", "VU");
    map.insert("681", "WF");
    map.insert("685", "WS");
    map.insert("967", "YE");
    map.insert("27", "ZA");
    map.insert("260", "ZM");
    map.insert("263", "ZW");
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_country_code_map() {
        let map = get_country_code_map();
        
        assert_eq!(map.get("33"), Some(&"FR"));
        assert_eq!(map.get("1"), Some(&"VI"));
        assert_eq!(map.get("44"), Some(&"JE"));
        assert_eq!(map.get("49"), Some(&"DE"));
        assert!(map.len() > 200); // Should have many country codes
    }

    #[test]
    fn test_status_code_parsing() {
        // Test status code parsing logic
        let taken_response = json!({"status_code": "TAKEN_NUMBER"});
        let ok_response = json!({"status_code": "OK"});
        let rate_limited_response = json!({"status_code": "RATE_LIMITED"});
        let unknown_response = json!({"status_code": "UNKNOWN"});
        
        let taken_status = taken_response.get("status_code").and_then(|s| s.as_str());
        let ok_status = ok_response.get("status_code").and_then(|s| s.as_str());
        let rate_limited_status = rate_limited_response.get("status_code").and_then(|s| s.as_str());
        let unknown_status = unknown_response.get("status_code").and_then(|s| s.as_str());
        
        assert_eq!(taken_status, Some(TAKEN_NUMBER_STATUS));
        assert_eq!(ok_status, Some(OK_STATUS));
        assert_eq!(rate_limited_status, Some("RATE_LIMITED"));
        assert_eq!(unknown_status, Some("UNKNOWN"));
    }

    #[tokio::test]
    async fn test_check_snapchat_invalid_country_code() {
        let client = Client::new();
        let result = check_snapchat("123456789", "999", &client).await;

        assert_eq!(result.name, "snapchat");
        assert!(!result.exists);
        assert!(result.rate_limit);
    }

}