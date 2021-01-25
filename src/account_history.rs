use crate::internal_prelude::*;
use crate::{Hash};
use chrono::Utc;

#[derive(Debug, Serialize)]
pub struct AccountHistoryRequest<'a> {
    action: &'a str,
    pub account: &'a Address,
    // #[serde(serialize_with = "serialize_to_string")]
    pub count: i32,
    pub raw: bool,
}

impl<'a> AccountHistoryRequest<'a> {
    pub fn new(account: &'a Address, count: i32) -> Self {
        Self {
            action: "account_history",
            account,
            count,
            raw: false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountHistoryResponse {
    pub account: Address,
    pub history: Vec<AccountHistoryEntry>,
    pub previous: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountHistoryEntry {
    #[serde(rename = "type")]
    block_type: String,
    account: Address,
    amount: Raw,
    local_timestamp: chrono::DateTime<Utc>,
    height: u64,
    hash: Hash,
}

impl NanoClient {
    pub async fn account_history(&self, request: &AccountHistoryRequest<'_>) -> Result<AccountHistoryResponse> {
        self.rpc(request).await
    }
}