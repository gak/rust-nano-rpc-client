mod internal_prelude;
mod key_expand;
mod raw;
mod send;
mod version;
mod wallet_add;
mod wallet_create;
mod wallet_balances;
mod receive_minimum_set;
mod account_weight;
mod account_representative;
mod account_balance;
mod account_history;

pub use key_expand::{KeyExpandRequest, KeyExpandResponse};
pub use raw::Raw;
pub use send::{SendRequest, SendResponse};
pub use version::VersionResponse;
pub use wallet_add::{WalletAddRequest, WalletAddResponse};
pub use wallet_create::{WalletCreateResponse};
pub use wallet_balances::{WalletBalancesRequest, WalletBalancesResponse};
pub use receive_minimum_set::{ReceiveMinimumSetRequest, ReceiveMinimumSetResponse};
pub use account_representative::{AccountRepresentativeRequest, AccountRepresentativeResponse};
pub use account_weight::{AccountWeightRequest, AccountWeightResponse};
pub use account_balance::{AccountBalanceRequest, AccountBalanceResponse};
pub use account_history::{AccountHistoryRequest, AccountHistoryResponse};
pub use bigdecimal;

use bigdecimal::ParseBigDecimalError;
use internal_prelude::*;
use serde;
use serde::de::DeserializeOwned;
use thiserror::Error;
use serde::{Serialize, Serializer};
use std::fmt::Display;

#[derive(Error, Debug)]
pub enum NanoError {
    #[error("RPC error")]
    RPCError(RPCError),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),

    #[error("Parse RPC error")]
    ParseRPCResponseError(serde_json::Error, String),

    #[error("Parse decimal error")]
    ParseDecimalError(#[from] ParseBigDecimalError),
}

type Result<T> = std::result::Result<T, NanoError>;

#[derive(Debug, Deserialize)]
pub struct RPCError {
    error: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Error(RPCError),
    Success(T),
}

pub struct NanoClient {
    address: String,
    port: u16,
}

impl NanoClient {
    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    async fn rpc<S: Serialize + ?Sized, R: DeserializeOwned + std::fmt::Debug>(
        &self,
        request: &S,
    ) -> Result<R> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("http://{}:{}/", self.address, self.port))
            .json(&request)
            .send()
            .await?;

        let text = res.text().await?;
        let response: Response<R> = match serde_json::from_str(&text) {
            Ok(r) => r,
            Err(e) => return Err(NanoError::ParseRPCResponseError(e, text)),
        };
        match response {
            Response::Success(r) => Ok(r),
            Response::Error(e) => Err(NanoError::RPCError(e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Address(String);

impl Address {
    // TODO: Parse and validate.
    pub fn from_str(s: &str) -> Self {
        Address(s.to_string())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet(String);

impl Wallet {
    // TODO: Parse and validate.
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Key(String);

impl Key {
    // TODO: Parse and validate.
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hash(String);

impl Hash {
    // TODO: Parse and validate.
    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub balance: Raw,
    pub pending: Raw,
}

// fn serialize_to_string<S, T>(x: &T, s: S) -> serde::Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//         T: Display,
// {
//     s.serialize_str(&format!("{}", x))
// }
//
#[cfg(test)]
mod tests {
    use crate::{NanoClient, AccountHistoryRequest, Address};

    #[tokio::test]
    async fn real_test() {
        let c = NanoClient::new("10.0.2.169".to_string(), 7076);
        let resp = c.version().await.unwrap();
        dbg!(resp);

        let address = Address::from_str("nano_3c9c41gmwsi9wysadpytw1pur5y3k7th6suj3aw966gfe6bga7i4daz9hb5e");
        let request = AccountHistoryRequest::new(&address, -1);
        let resp = c.account_history(&request).await.unwrap();
        dbg!(resp);
    }
}
