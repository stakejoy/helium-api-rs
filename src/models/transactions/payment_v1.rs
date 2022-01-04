use crate::models::Hnt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PaymentV1 {
    pub hash: String,
    pub height: u64,
    pub time: i64,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    pub fee: u64,
    pub nonce: u64,
    pub payer: String,
    pub payee: String,
}
