use std::collections::HashMap;

use reqwest::Url;

use crate::file::TorrentFile;

use super::http::{HttpTracker, HttpTrackerRquest};

#[derive(Debug, Clone)]
pub struct Tracker {
    pub torrent_file: TorrentFile,
}

impl Tracker {
    pub fn new(tf: TorrentFile) -> Self {
        Self { torrent_file: tf }
    }
    /// used to send tracker requests
    pub async fn get_peers(&self) {
        let http_tracker = HttpTracker::new();
        let res_map: HashMap<String, String> = HashMap::new();
        // multiple file processing
        if self.torrent_file.is_multiple_files {
            let files = self.torrent_file.meta_data.info.files.clone().unwrap();
            for (index, file) in files.iter().enumerate() {
                for (_i, url) in self.torrent_file.announces.iter().enumerate() {
                    let protocol_str: Vec<&str> = url.split("://").collect();
                    match protocol_str[0] {
                        "udp" => {}
                        "http" | "https" => {
                            let req = HttpTrackerRquest {
                                info_hash: self.torrent_file.info_hash.clone(),
                                peer_id: "".to_string(),
                                port: "6881".to_string(),
                                uploaded: "0".to_string(),
                                downloaded: "0".to_string(),
                                left: file.length.to_string(),
                                compact: "".to_string(),
                                event: "".to_string(),
                            };
                            let curl =
                                Url::parse_with_params(url, req.to_request_params()).unwrap();
                            match http_tracker.http_request(&curl.to_string(), req).await {
                                Ok(r) => {
                                    break;
                                }
                                Err(e) => {
                                    continue;
                                }
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
        } else {
            // try sending tracker requests to all alternate addresses
            for (_i, url) in self.torrent_file.announces.iter().enumerate() {
                let protocol_str: Vec<&str> = url.split("://").collect();
                match protocol_str[0] {
                    "udp" => {}
                    "http" | "https" => {}
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
}
