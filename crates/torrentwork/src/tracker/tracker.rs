use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use tokio::{join, task::JoinHandle};

use crate::file::TorrentFile;

use super::http::{HttpTracker, HttpTrackerResponse, HttpTrackerRquest};

const HTTP_TRACKER_COMPACT_MODE: (&str, &str) = ("0", "1");
const HTTP_TRACKER_PORT: &str = "6881";
const HTTP_TRACKER_EVENT_MODE: (&str, &str, &str) = ("started", "completed", "stopped");
const PEER_ID: &str = "00112233445566778899";

#[derive(Debug, Clone)]
pub struct Tracker {
    pub torrent_file: TorrentFile,
}

impl Tracker {
    pub fn new(tf: TorrentFile) -> Self {
        Self { torrent_file: tf }
    }
    /// used to send tracker requests
    pub async fn get_peers(&mut self) -> Result<HashMap<String, Vec<HttpTrackerResponse>>, String> {
        let res_map: Arc<Mutex<HashMap<String, Vec<HttpTrackerResponse>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        // multiple file processing
        if self.torrent_file.is_multiple_files {
            // task pool
            let mut task_pool: Vec<JoinHandle<_>> = Vec::new();
            let mut files = self.torrent_file.meta_data.info.files.clone().unwrap();
            for (_index, file) in files.iter_mut().enumerate() {
                for (_i, u) in self.torrent_file.announces.iter().enumerate() {
                    let protocol_str: Vec<&str> = u.split("://").collect();
                    match protocol_str[0] {
                        "udp" => {}
                        "http" | "https" => {
                            let req = HttpTrackerRquest::new(
                                self.torrent_file.info_hash.clone(),
                                PEER_ID.to_string(),
                                HTTP_TRACKER_PORT.to_string(),
                                "0".to_string(),
                                "0".to_string(),
                                file.length.to_string(),
                                HTTP_TRACKER_COMPACT_MODE.0.to_string(),
                                HTTP_TRACKER_EVENT_MODE.0.to_string(),
                            );
                            // url
                            let url = u.clone();
                            // fiel sha1 hash
                            let file_sha1_hash = file.to_sha1_hash();
                            let arc_map = Arc::clone(&res_map);
                            task_pool.push(tokio::spawn(async move {
                                // http tracker
                                let http_tracker = HttpTracker::new();
                                match http_tracker.send(&url, req).await {
                                    Ok(r) => {
                                        let mut map = arc_map.lock().unwrap();
                                        if map.contains_key(&file_sha1_hash) {
                                            map.get_mut(&file_sha1_hash).unwrap().push(r);
                                        } else {
                                            map.insert(file_sha1_hash, vec![r]);
                                        }
                                    }
                                    Err(_e) => {}
                                }
                            }));
                        }
                        _ => {
                            continue;
                        }
                    }
                }
            }
            for t in task_pool {
                join!(t);
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
        let arc_map = Arc::clone(&res_map);
        let ret_map = arc_map.lock().unwrap().to_owned();
        Ok(ret_map)
    }
}
