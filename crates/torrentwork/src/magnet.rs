use crate::file::TorrentFile;

pub struct Magnet {
    torrent_file: TorrentFile,
}

impl Magnet {
    pub fn new(url: &str) -> Result<String, String> {
        match url {
            "1" => Ok("1".to_string()),
            "2" => Err("1".to_string()),
            _ => Ok("1".to_string()),
        }
    }
}
