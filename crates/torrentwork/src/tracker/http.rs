use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_bencode::de;

pub struct HttpTracker {
    client: reqwest::Client,
}

impl HttpTracker {
    pub fn new() -> Self {
        // let client = reqwest::Client::new();
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(5)) // time out 5s
                .build()
                .unwrap(),
        }
    }
    /// http tracker requests
    pub async fn http_request(
        &self,
        url: &str,
        req: HttpTrackerRquest,
    ) -> Result<HttpTrackerResponse, String> {
        match self.client.get(url.to_string()).send().await {
            Ok(r) => {
                let s = r.text().await.unwrap();
                match de::from_str::<HttpTrackerResponse>(&s) {
                    Ok(thr) => Ok(thr),
                    Err(e) => Err(format!(
                        "tracker http response serialization failed {:?}",
                        e
                    )),
                }
            }
            Err(e) => Err(format!("http request bad request {:?}", e)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpTrackerRquest {
    pub info_hash: String,
    pub peer_id: String,
    pub port: String,
    pub uploaded: String,
    pub downloaded: String,
    pub left: String,
    pub compact: String,
    pub event: String,
}
impl HttpTrackerRquest {
    pub fn to_request_params(&self) -> Vec<(&str, String)> {
        vec![
            ("info_hash", self.info_hash.clone()),
            ("peer_id", "".to_string()),
            ("port", "".to_string()),
            ("uploaded", "0".to_string()),
            ("downloaded", "0".to_string()),
            ("left", self.left.clone()),
            ("compact", "1".to_string()),
            ("event", "started".to_string()),
        ]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpTrackerResponse {
    pub complete: Option<u64>,
    pub downloaded: Option<u64>,
    pub incomplete: Option<u64>,
    pub interval: Option<u64>,
    #[serde(rename = "min interval")]
    pub min_interval: Option<u64>,
    pub peers: Vec<Peer>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Peer {
    #[serde(rename = "peer id")]
    pub peer_id: u16,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub port: Option<u16>,
}
