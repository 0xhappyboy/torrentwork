use std::{
    process,
    sync::{Arc, Mutex},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::{
    peer::{Handshake, Peer},
    torrent::File,
    tracker::tracker::Tracker,
};

/// the callback function type when the download task progress is updated.
pub type FnUpdateProgressCallBack = fn(process: u16);

/// download task abstract structure
#[derive(Clone, Debug)]
pub struct DownloadTask {
    // files to be downloaded
    file: File,
    // tracker
    tracker: Tracker,
    // the peer list at the current progress
    peers: Option<Vec<Peer>>,
    // download progress in percentage
    pub progress: Arc<Mutex<u16>>,
    // the number of bytes downloaded.
    pub downloaded: Arc<Mutex<u64>>,
    // total downloads
    pub total_download: i64,
    // file download status
    pub status: DownloadStatus,
    // info hash
    pub info_hash: String,
}

impl DownloadTask {
    pub fn new(info_hash: String, file: File, tracker: Tracker) -> Self {
        Self {
            file: file.clone(),
            peers: None,
            progress: Arc::new(Mutex::new(0)),
            status: DownloadStatus::WAITING,
            tracker: tracker,
            downloaded: Arc::new(Mutex::new(0)),
            total_download: file.clone().length,
            info_hash: info_hash,
        }
    }
    pub async fn start(&mut self) {
        self.status = DownloadStatus::DOWNLOADING;
        self.peers = Some(self.peers().await.unwrap());
        for (a, b) in self.peers.clone().unwrap().iter().enumerate() {
            let add = b.to_address();
            b.handshake().await;
        }
    }
    // get the peer list based on the current download progress
    pub async fn peers(&mut self) -> Result<Vec<Peer>, String> {
        let mut peers = Vec::<Peer>::new();
        match self
            .tracker
            .get_peers(self.file.clone(), self.downloaded(), 0)
            .await
        {
            Ok(r_vec) => {
                r_vec.iter().for_each(|r| {
                    if !r.peers.is_none() {
                        peers.append(&mut r.peers.clone().unwrap());
                    }
                });
            }
            Err(e) => return Err(e),
        }
        Ok(peers)
    }
    // set progress
    pub async fn update_progress(&mut self, new_progress: u16) {
        let progress = Arc::clone(&self.progress);
        let mut m_progress = progress.lock().unwrap();
        *m_progress += new_progress;
    }
    pub fn downloaded(&self) -> u64 {
        let arc = Arc::clone(&self.downloaded);
        let m = arc.lock().unwrap();
        *m
    }
}

#[derive(Debug, Clone, Copy)]
/// enumeration identifying the download status
pub enum DownloadStatus {
    WAITING,
    DOWNLOADING,
    FINISH,
}
