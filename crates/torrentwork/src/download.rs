use crate::{peer::Peer, torrent::File};

#[derive(Clone, Debug)]
pub struct Download {
    pub file: File,
    pub peers: Vec<Peer>,
    pub progress: u16,
}

impl Download {
    pub fn new(file: File, peers: Vec<Peer>, progress: u16) -> Self {
        Self {
            file: file,
            peers: peers,
            progress: 0,
        }
    }
    pub fn start() {
        // tokio::spawn(async move {
        //     loop {
        //         let listener = TcpListener::bind("127.0.0.1:6881").await.unwrap();
        //         println!("start listener");
        //         let (mut socket, _) = listener.accept().await.unwrap();
        //     }
        // });
    }
    pub fn download_by_http() {}
    pub fn download_by_udp() {}
}
