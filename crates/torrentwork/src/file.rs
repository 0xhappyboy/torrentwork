use magnet_url::Magnet;
use serde_bencode::de;
use sha1::{Digest, Sha1};
use urlencoding::encode;
extern crate url;
use crate::torrent::Torrent;

/// torrent file abstract
#[derive(Clone, Debug)]
pub struct TorrentFile {
    /// torrent file abstract
    pub meta_data: Torrent,
    /// torrent file all tracker server address
    pub announces: Vec<String>,
    /// torrent file info hash
    pub info_hash: String,
    /// multiple file tags
    pub is_multiple_files: bool,
}

impl TorrentFile {
    /// constructs a TorrentFile instance using the path to a local .torrent file.
    pub fn new(file_path: String) -> Result<Self, String> {
        match std::fs::read(file_path) {
            Ok(buf) => match de::from_bytes::<Torrent>(&buf) {
                Ok(t) => {
                    // Save the result of sha1(bencode(info)) to send Tracker request.
                    let mut info_bytes = serde_bencode::to_bytes(&t.info).unwrap();
                    let mut s1 = Sha1::new();
                    s1.update(info_bytes);
                    let mut info_sha1_hash = hex::encode(s1.finalize().to_vec());
                    // trim 20 bytes
                    info_sha1_hash = (&info_sha1_hash[..20]).to_string();
                    Ok(Self {
                        meta_data: t.clone(),
                        announces: Self::get_all_announce(&t),
                        info_hash: encode(&info_sha1_hash).to_string(),
                        is_multiple_files: Self::is_multiple_files(&t),
                    })
                }
                Err(e) => Err(format!("ERROR: {:?}", e).to_string()),
            },
            Err(e) => Err(format!("ERROR: {:?}", e).to_string()),
        }
    }
    /// get .torrent file all announce url
    fn get_all_announce(torrent: &Torrent) -> Vec<String> {
        let mut announces: Vec<String> = vec![];
        if !torrent.announce.is_none() {
            announces.push(torrent.announce.clone().unwrap());
        }
        for (_i, v) in torrent.announce_list.clone().unwrap().iter().enumerate() {
            for (_index, value) in v.iter().enumerate() {
                announces.push(value.to_string());
            }
        }
        announces
    }
    /// make a magnet url
    pub fn make_magnet_url(&self) -> Result<String, String> {
        let m = Magnet {
            dn: Some(self.meta_data.info.name.clone()),
            hash_type: Some("btih".to_string()),
            xt: Some(self.info_hash.clone()),
            xs: None,
            kt: None,
            ws: None,
            acceptable_source: None,
            mt: None,
            xl: None,
            tr: vec![],
        };
        Ok(m.to_string())
    }
    /// make a .torrent file
    pub fn make() {}

    /// storage the .torrent to disk
    pub fn storage(&self) {}
    /// is multiple file
    pub fn is_multiple_files(tf: &Torrent) -> bool {
        if tf.info.files.is_none() { false } else { true }
    }
}
