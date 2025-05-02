pub struct UDPTracker {}

impl UDPTracker {
    /// udp tracker requests
    async fn udp_request(&self, url: &str) -> Result<(), reqwest::Error> {
        Ok(())
    }
}

struct TrackerUDPRquest {
    transaction_id: String,
    connection_id: String,
    event: String,
    num_want: u64,
    leechers: u64,
    seeders: u64,
}

struct TrackerUDPResponse {}
