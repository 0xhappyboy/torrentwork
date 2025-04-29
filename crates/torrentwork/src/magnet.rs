use regex::Regex;
use sha1_checked::digest::consts::False;

use crate::file::TorrentFile;

pub struct Magnet {
    torrent_file: TorrentFile,
}

impl Magnet {
    pub fn new(url: &str) -> Result<String, String> {
        // verify magnet protocol format
        let r = Regex::new(r"magnet:\?xt=urn:btih:[0-9a-fA-F]{40,}.*").unwrap();
        match r.is_match(url) {
            true => Ok("1".to_string()),
            False => Err("link format error".to_string()),
            _ => Ok("1".to_string()),
        }
    }
}
