use std::{sync::Arc, time::Duration};

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_bencode::de;
use url::Url;

#[derive(Debug, Clone)]
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
        let curl = Url::parse_with_params(url, req.to_request_params()).unwrap();
        match self.client.get(curl.to_string()).send().await {
            Ok(r) => match r.status() {
                StatusCode::OK => {
                    let s = r.text().await.unwrap();
                    match de::from_str::<HttpTrackerResponse>(&s) {
                        Ok(thr) => Ok(thr),
                        Err(e) => Err(format!(
                            "tracker http response serialization failed {:?}",
                            e
                        )),
                    }
                }
                _ => Err(format!("tracker server exception response {:?}", r)),
            },
            Err(e) => Err(format!("tracker server bad request {:?}", e)),
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
    pub fn new(
        info_hash: String,
        peer_id: String,
        port: String,
        uploaded: String,
        downloaded: String,
        left: String,
        compact: String,
        event: String,
    ) -> Self {
        let req = HttpTrackerRquest {
            info_hash: info_hash,
            peer_id: peer_id,
            port: port,
            uploaded: uploaded,
            downloaded: downloaded,
            left: left,
            compact: compact,
            event: event,
        };
        req
    }
    pub fn to_request_params(&self) -> Vec<(&str, String)> {
        vec![
            ("info_hash", self.info_hash.clone()),
            ("peer_id", self.peer_id.to_string()),
            ("port", self.port.to_string()),
            ("uploaded", self.uploaded.to_string()),
            ("downloaded", self.downloaded.to_string()),
            ("left", self.left.clone()),
            ("compact", self.compact.to_string()),
            ("event", self.event.to_string()),
        ]
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpTrackerResponse {
    pub complete: Option<u64>,
    pub downloaded: Option<u64>,
    pub incomplete: Option<u64>,
    pub interval: Option<u64>,
    #[serde(rename = "min interval")]
    pub min_interval: Option<u64>,
    pub peers: Option<Vec<Peer>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    #[serde(default, rename = "peer id")]
    pub peer_id: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub port: Option<u64>,
}

impl Peer {}
