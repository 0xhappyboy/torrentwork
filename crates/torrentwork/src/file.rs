use reqwest::{Client, Proxy};
use reqwest::{Url, header};
use serde::Deserialize;
use serde::Serialize;
use serde_bencode::de;
use serde_bytes::ByteBuf;
use sha1_checked::Sha1;
use std::io::{self, Read};

use crate::tracker::Tracker;

#[derive(Clone)]
pub struct TorrentFile {
    pub torrent: Torrent,
    pub tracker: Tracker,
}

impl TorrentFile {
    /// constructs a TorrentFile instance using the path to a local .torrent file.
    pub fn new(file_path: String) -> Result<Self, String> {
        match std::fs::read(file_path) {
            Ok(buf) => match de::from_bytes::<Torrent>(&buf) {
                Ok(mut t) => {
                    // Save the result of sha1(bencode(info)) to send Tracker request.
                    let info_ben = serde_bencode::to_bytes(&t.info).unwrap();
                    let result = Sha1::try_digest(info_ben);
                    let info_sha1_hash = hex::encode(result.hash());
                    t.set_info_hash(info_sha1_hash);
                    Ok(Self {
                        torrent: t.clone(),
                        tracker: Tracker { torrent: t.clone() },
                    })
                }
                Err(e) => Err(format!("ERROR: {:?}", e).to_string()),
            },
            Err(e) => Err(format!("ERROR: {:?}", e).to_string()),
        }
    }
    /// is multiple file
    pub fn is_multiple_files(&self) -> bool {
        let torrent = &self.torrent;
        if torrent.info.files.is_none() {
            false
        } else {
            true
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Node(String, i64);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct File {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
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
}

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
    pub info_hash: Option<String>,
}

impl Torrent {
    pub fn set_info_hash(&mut self, hash: String) {
        self.info_hash = Some(hash);
    }
}
