use bitcoin::hashes::sha256::Hash;
use cln_rpc::primitives::Amount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Htlc {
    pub short_channel_id: u64,
    #[serde(alias = "amount_msat")]
    pub amount_msat: Amount,
    pub cltv_expiry: u32,
    pub cltv_expiry_relative: u32,
    pub payment_hash: Hash,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Onion {
    pub payload: String,
    pub short_channel_id: Option<u64>,
    #[serde(alias = "forward_msat")]
    pub forward_msat: Amount,
    pub outgoing_cltv_value: u32,
    pub shared_secret: Hash,
    pub next_onion: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HtlcAccepted {
    pub htlc: Htlc,
    pub onion: Onion,
}
