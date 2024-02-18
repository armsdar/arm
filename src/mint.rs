use bitcoincore_rpc::bitcoin::Txid;
use bitcoincore_rpc::Client;

pub struct Mint {
    pub p: String,
    pub op: String,
    pub tick: String,
    pub amt: String,
}

impl Mint {
    pub async fn new(tick: &str, amt: u32) ->Self {
        Self{
            p: "brc-20".to_string(),
            op: "mint".to_string(),
            tick: tick.to_string(),
            amt: amt.to_string(),
        }
    }
}