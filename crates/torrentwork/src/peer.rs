use bincode::{Decode, Encode, config};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    #[serde(default, rename = "peer id")]
    pub peer_id: Option<String>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub port: Option<u64>,
    #[serde(skip)]
    pub info_hash: String,
}

impl Peer {
    pub async fn handshake(&self) -> Result<String, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(self.to_address()).await;
        match stream {
            Ok(mut s) => {
                let h = Handshake::new(self.info_hash.clone(), self.peer_id.clone().unwrap());
                match s.write_all(&h.to_bytes()).await {
                    Ok(()) => {
                        let mut buffer = [0; 1024];
                        // Read response from server
                        let len = s.read(&mut buffer).await.unwrap();
                        let response = std::str::from_utf8(&buffer[..len])?;
                    }
                    Err(e) => {}
                }
            }
            Err(_e) => {}
        }
        Ok("".to_string())
    }
    pub fn to_address(&self) -> String {
        format!("{}:{}", self.ip.clone().unwrap(), self.port.unwrap())
    }
}

const BITTORRENT_PROTOCOL_IDENTIFIER: &str = "BitTorrent protocol";

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Encode, Decode)]
pub struct Handshake {
    /// protocol identifier, always "BitTorrent protocol", 19 bytes.
    pub protocol: [u8; 19],
    /// reserved fields, 8 bytes.
    pub reserved: [u8; 8],
    /// torrent file metadata "Info" field Sha1 hash value, 20 bytes.
    pub info_hash: [u8; 20],
    /// custom peer id, 20 bytes.
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: String, peer_id: String) -> Self {
        // protocol identifier bytes
        let mut p: [u8; 19] = [0; 19];
        let mut hash: [u8; 20] = [0; 20];
        let mut p_id: [u8; 20] = [0; 20];
        p.copy_from_slice(&BITTORRENT_PROTOCOL_IDENTIFIER.as_bytes()[..19]);
        hash.copy_from_slice(&info_hash.as_bytes()[..20]);
        p_id.copy_from_slice(&peer_id.as_bytes()[..20]);
        Self {
            protocol: p,
            info_hash: hash,
            peer_id: p_id,
            reserved: [0; 8],
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let config = config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(self, config).unwrap();
        bytes
    }
}

pub struct Message {
    // message length
    pub length: [u8; 4],
    // message type
    pub msg_type: [u8; 1],
}

impl Message {}

pub enum MessageType {
    MsgChoke = 0,
    MsgUnchoke = 1,
    MsgInterested = 2,
    MsgNotInterested = 3,
    MsgHave = 4,
    MsgBitfield = 5,
    MsgRequest = 6,
    MsgPiece = 7,
    MsgCancel = 8,
}
