use serde_json::Value;

use reqwest::blocking;
use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subdomains {
    pub success: bool,
    pub count: usize,
    pub subdomains: Vec<Subdomain>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subdomain {
    pub subdomain: String,
}

pub async fn SubdmainGetURlApi(url: &str) -> Option<Subdomains> {
    let get = reqwest::get(format!(
        "https://api.c99.nl/subdomainfinder?key=KeyAqui&realtime=true&domain={}", // entrar em contato com o dono
        url.replace(" ", "")
    ))
    .await;

    match get {
        Ok(convert_to_json) => {
            let text = convert_to_json.text().await.unwrap_or("Error".to_string());
            if let Ok(mut json1) = serde_json::from_str::<Subdomains>(&text) {
                json1.subdomains.push(Subdomain {
                    subdomain: url.to_string(),
                });
                return Some(json1);
            }
        }
        _ => {}
    }
    None
}
