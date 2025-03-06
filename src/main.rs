use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
mod config;
use config::CONFIG;

// const CLOUDFLARE_ZONE_ID: &str = "your_zone_id";
// const CLOUDFLARE_API_TOKEN: &str = "your_api_token";
// const DNS_RECORD_ID: &str = "your_dns_record_id";
// const DNS_NAME: &str = "your.domain.com";

#[derive(Deserialize)]
struct IpResponse {
    ip: String,
}

#[derive(Serialize,Deserialize, Debug)]
struct DnsUpdateRequest {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

async fn get_public_ip() -> Option<String> {
    let res = reqwest::get("https://api.ipify.org?format=json")
        .await
        .ok()?;
    let json: IpResponse = res.json().await.ok()?;
    Some(json.ip)
}

async fn update_dns_record(client: &Client, new_ip: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        CONFIG.cloudflare_zone_id,
        CONFIG.dns_record_id
    );

    let request = DnsUpdateRequest {
        record_type: "A".into(),
        name: CONFIG.dns_name.clone(),
        content: new_ip.into(),
        ttl: 120,
        proxied: false,
    };


    let response = client
        .patch(&url)
        .bearer_auth(CONFIG.cloudflare_api_token.clone())
        .json(&request)
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        println!("✅ Update dns update success! {}", new_ip);
        println!("🔗 {}.{}", CONFIG.dns_name,CONFIG.cloudflare_domain);
    } else {
        println!("❌ Error when update dns {:?}", response.text().await?);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let mut last_ip = String::new();
    loop {
        if let Some(ip) = get_public_ip().await {
            println!("🌐 public ip: {}", ip);
            if ip != last_ip {
                println!("🔄 Ip change: {}", ip);
                if let Err(err) = update_dns_record(&client, &ip).await {
                    eprintln!("⚠ Error while update Cloudflare: {:?}", err);
                } else {
                    last_ip = ip;
                }
            }
        }
        sleep(Duration::from_secs(60)).await;
    }
}
