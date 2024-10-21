use reqwest::header::{AUTHORIZATION, HeaderMap};
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
struct KVKey {
    name: String,
    metadata: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct KVKeyResponse {
    result: Vec<KVKey>,
}

async fn list_kv_keys(account_id: &str, namespace_id: &str, auth_token: &str) -> Result<KVKeyResponse, reqwest::Error> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/keys",
        account_id, namespace_id
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", auth_token).parse().unwrap());

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await?
        .json::<KVKeyResponse>()
        .await?;

    Ok(response)
}

async fn get_kv_value(account_id: &str, namespace_id: &str, auth_token: &str, key: &str) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/storage/kv/namespaces/{}/values/{}",
        account_id, namespace_id, key
    );

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("Bearer {}", auth_token).parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.get(&url).headers(headers).send().await?.text().await?;
    
    Ok(response)
}

async fn download_kv(filter: &str, account_id: &str, namespace_id: &str, auth_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keys = list_kv_keys(account_id, namespace_id, auth_token).await?;

    // Ensure the "kv-data" directory exists
    let kv_data_dir = Path::new("kv-data");
    if !kv_data_dir.exists() {
        fs::create_dir(kv_data_dir)?;
    }

    for kv in keys.result {
        // Check if the key name contains the filter substring
        if !kv.name.contains(filter) {
            // println!("Skipping KV key: {}", kv.name);
            continue;
        }

        let filename = format!("kv-data/{}.json", kv.name);
        let kv_value = get_kv_value(account_id, namespace_id, auth_token, &kv.name).await?;

        fs::write(&filename, kv_value)?;
        println!("Saved KV: {}", kv.name);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the filter argument from the command line (default is empty string)
    let args: Vec<String> = env::args().collect();
    let filter = if args.len() > 1 { &args[1] } else { "" };

    // Retrieve environment variables from .env
    let account_id = env::var("ACCOUNT_ID").expect("ACCOUNT_ID must be set");
    let namespace_id = env::var("NAMESPACE_ID").expect("NAMESPACE_ID must be set");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set");

    // Download KV data
    if let Err(e) = download_kv(filter, &account_id, &namespace_id, &auth_token).await {
        eprintln!("Error: {}", e);
    }
}
