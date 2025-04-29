use reqwest::{Client, Proxy};
use reqwest::{Url, header};
use serde::Deserialize;
use serde::Serialize;
use serde_bencode::de;
use serde_bytes::ByteBuf;
use sha1_checked::Sha1;
use std::io::{self, Read};

#[derive(Clone)]
pub struct TorrentFile {
    pub torrent: Torrent,
    pub info_hash: String,
}

impl TorrentFile {
    /// constructs a TorrentFile instance using the path to a local .torrent file.
    pub fn new(file_path: String) -> Result<Self, String> {
        match std::fs::read(file_path) {
            Ok(buf) => match de::from_bytes::<Torrent>(&buf) {
                Ok(t) => {
                    // Save the result of sha1(bencode(info)) to send Tracker request.
                    let info_ben = serde_bencode::to_bytes(&t.info).unwrap();
                    let result = Sha1::try_digest(info_ben);
                    let info_sha1_hash = hex::encode(result.hash());
                    Ok(Self {
                        torrent: t,
                        info_hash: info_sha1_hash,
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
    /// total block num
    pub fn total_block_num(&self) -> i64 {
        self.torrent.info.length.unwrap() / self.torrent.info.piece_length
    }
    // download
    pub fn download(&self) -> Result<&str, &str> {
        match self.is_multiple_files() {
            true => {}
            false => {}
        }
        Ok("")
    }
    pub async fn download_by_http(&self) -> Result<(), reqwest::Error> {
        let mut params = vec![
            ("info_hash", self.info_hash.clone()),
            ("peer_id", "".to_string()),
            ("port", "".to_string()),
            ("uploaded", "0".to_string()),
            ("downloaded", "0".to_string()),
            ("left", "0".to_string()),
            ("event", "started".to_string()),
        ];
        // let client = Client::builder()
        //     .proxy(Proxy::http("http://localhost:7897")?)
        //     .build()?;
        let client = reqwest::Client::new();
        let url = self.torrent.announce.clone().unwrap();
        let u = self.torrent.announce.clone().unwrap();
        let url = Url::parse_with_params(&url, &params).unwrap();
        let body = client.get(url).send().await?;
        Ok(())
    }
    pub fn download_by_udp() {}
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
