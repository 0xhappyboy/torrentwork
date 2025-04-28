pub struct TorrentFile {
    torrent: Torrent,
}

impl TorrentFile {
    /// constructs a TorrentFile instance using the path to a local .torrent file.
    pub fn new(file_path: String) -> Result<Self, String> {
        match std::fs::read(file_path) {
            Ok(buf) => match de::from_bytes::<Torrent>(&buf) {
                Ok(t) => Ok(Self { torrent: t }),
                Err(e) => Err(format!("ERROR: {:?}", e).to_string()),
            },
            Err(e) => Err(format!("ERROR: {:?}", e).to_string()),
        }
    }
    pub fn downlaod(&self) -> Result<&str, &str> {
        todo!();
        Ok("")
    }
}

use serde::Deserialize;
use serde_bencode::de;
use serde_bytes::ByteBuf;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct Node(String, i64);

#[derive(Debug, Deserialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Info {
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

#[derive(Debug, Deserialize)]
struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
}
