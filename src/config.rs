// use clap::{Parser, arg};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;
pub struct Config {
    // #[arg(long, env = "SSH_PORT", default_value = "2222")]
    pub ssh_port: u16,
    pub ssh_user: String,
    pub ssh_host: String,
    pub fail2ban_max_retry: u32,
    pub fail2ban_ban_time: u32,
    pub cloudflare_tunnel_id: String,
    pub cloudflare_auth_key: String,
    pub cloudflare_domain: String,
    pub cloudflare_zone_id: String,
    pub cloudflare_api_token: String,
    pub dns_record_id: String,
    pub dns_name: String,
}

// Biến global chứa config, chỉ khởi tạo một lần
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok(); // Load file .env nếu có

    Config {
        ssh_port: env::var("SSH_PORT")
            .unwrap_or_else(|_| "2222".to_string())
            .parse()
            .unwrap(),
        ssh_user: env::var("SSH_USER").unwrap_or_else(|_| "myuser".to_string()),
        ssh_host: env::var("SSH_HOST").unwrap_or_else(|_| "myserver.com".to_string()),
        fail2ban_max_retry: env::var("FAIL2BAN_MAX_RETRY")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .unwrap(),
        fail2ban_ban_time: env::var("FAIL2BAN_BAN_TIME")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()
            .unwrap(),
        cloudflare_tunnel_id: env::var("CLOUDFLARE_TUNNEL_ID").unwrap_or_else(|_| "".to_string()),
        cloudflare_auth_key: env::var("CLOUDFLARE_AUTH_KEY").unwrap_or_else(|_| "".to_string()),
        cloudflare_domain: env::var("CLOUDFLARE_DOMAIN").unwrap_or_else(|_| "".to_string()),
        cloudflare_zone_id: env::var("CLOUDFLARE_ZONE_ID").unwrap_or_else(|_| "".to_string()),
        cloudflare_api_token: env::var("CLOUDFLARE_API_TOKEN").unwrap_or_else(|_| "".to_string()),
        dns_record_id: env::var("DNS_RECORD_ID").unwrap_or_else(|_| "".to_string()),
        dns_name: env::var("DNS_NAME").unwrap_or_else(|_| "".to_string()),
    }
});
