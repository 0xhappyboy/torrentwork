use std::time::Duration;

use log::{error, info, warn};
use reqwest::{Client, Response, StatusCode, Url};

use crate::file::Torrent;

#[derive(Debug, Clone)]
pub struct Tracker {
    pub torrent: Torrent,
}

impl Tracker {
    /// used to send tracker requests
    pub async fn request(&self) {
        // let client = reqwest::Client::new();
        let client = Client::builder()
            .timeout(Duration::from_secs(5)) // time out 5s
            .build()
            .unwrap();
        let mut flag = false;
        let source_url_list = self.torrent.announce_list.clone().unwrap();
        // try sending tracker requests to all alternate addresses
        for (i, v) in source_url_list.iter().enumerate() {
            for (index, value) in v.iter().enumerate() {
                let protocol_str: Vec<&str> = value.split("://").collect();
                match protocol_str[0] {
                    "udp" => {}
                    "http" | "https" => match self.http_request(&client, &value).await {
                        Ok(r) => {
                            match r.status() {
                                StatusCode::OK => {
                                    // tracker response processing
                                    // info!(target: "SUCCESS", "tracker request successful {:?}", r);
                                    flag = true;
                                    break;
                                }
                                StatusCode::BAD_REQUEST => {
                                    error!(target: "ERROR", "http request bad request {:?}", r);
                                    continue;
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            error!(target: "ERROR", "http request time out {:?}", e);
                            continue;
                        }
                    },
                    _ => {
                        warn!(target: "WARN", "unknown tracker service protocol");
                        continue;
                    }
                }
            }
            if flag {
                break;
            }
        }
    }
    /// http tracker requests
    async fn http_request(
        &self,
        client: &reqwest::Client,
        url: &str,
    ) -> Result<Response, reqwest::Error> {
        let url = Url::parse_with_params(
            url,
            &TrackerHttpRquest::build_get_request_params(&self.torrent),
        )
        .unwrap();
        let r: Response = client.get(url.to_string()).send().await?;
        Ok(r)
    }
    /// udp tracker requests
    async fn udp_request(&self, client: &reqwest::Client, url: &str) -> Result<(), reqwest::Error> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct TrackerHttpRquest {}
impl TrackerHttpRquest {
    pub fn build_get_request_params(torrent: &Torrent) -> Vec<(&str, String)> {
        vec![
            ("info_hash", torrent.info_hash.as_ref().unwrap().to_string()),
            ("peer_id", "00112233445566778899".to_string()),
            ("port", "6881".to_string()),
            ("uploaded", "0".to_string()),
            ("downloaded", "0".to_string()),
            ("left", "456456".to_string()),
            ("event", "started".to_string()),
        ]
    }
}

#[derive(Debug, Clone)]
struct TrackerHttpResponse {}

#[derive(Debug, Clone)]
struct TrackerUDPRquest {}

#[derive(Debug, Clone)]
struct TrackerUDPResponse {}
