use serde::Deserialize;
use serde::Serialize;
use serde_bytes::ByteBuf;

use crate::tracker::http::HttpTrackerResponse;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub info: Info,
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Node(String, i64);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    pub tracker_response_list: Option<Vec<HttpTrackerResponse>>,
}

impl File {
    pub fn set_tracker_response_list(&mut self, l: Option<Vec<HttpTrackerResponse>>) {
        self.tracker_response_list = l;
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
    pub tracker_response_list: Option<Vec<HttpTrackerResponse>>,
}

impl Info {
    pub fn set_tracker_response_list(&mut self, l: Option<Vec<HttpTrackerResponse>>) {
        self.tracker_response_list = l;
    }
    pub fn set_files(&mut self, files: Vec<File>) {
        self.files = Some(files);
    }
}
