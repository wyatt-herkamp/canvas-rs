use reqwest::header::{AUTHORIZATION, USER_AGENT};
use reqwest::{Client, ClientBuilder};

pub const URL: Option<&str> = option_env!("CANVAS_URL");
pub const TOKEN: Option<&str> = option_env!("CANVAS_TOKEN");
pub const USER_AGENT_VALUE: &str = "Canvas API Rust Client";

pub fn build_client() -> Client {
    if URL.is_none() || TOKEN.is_none() {
        panic!("CANVAS_URL and CANVAS_TOKEN must be set");
    }

    let auth_header = format!("Bearer {}", TOKEN.unwrap());
    let mut default_headers = reqwest::header::HeaderMap::new();
    default_headers.append(
        AUTHORIZATION,
        auth_header.parse().expect("Failed to parse auth header"),
    );
    default_headers.append(
        USER_AGENT,
        USER_AGENT_VALUE
            .parse()
            .expect("Failed to parse user agent header"),
    );
    ClientBuilder::new()
        .default_headers(default_headers)
        .build()
        .expect("Failed to build client")
}
pub fn get_url(path: &str) -> String {
    format!("{}/api/v1/{}", URL.unwrap(), path)
}
