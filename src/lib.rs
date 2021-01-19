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

use bigdecimal::ParseBigDecimalError;
use internal_prelude::*;
use serde;
use serde::de::DeserializeOwned;
use thiserror::Error;

pub struct NanoClient {
    address: String,
    port: u16,
}

#[derive(Error, Debug)]
pub enum NanoError {
    #[error("RPC error")]
    RPCError(RPCError),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),

    #[error("Parse error")]
    ParseError(#[from] ParseBigDecimalError),
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

        let response = res.json::<Response<R>>().await?;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet(String);

impl Wallet {
    // TODO: Parse and validate.
    pub fn from_str(s: &str) -> Self {
        Wallet(s.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key(String);

impl Key {
    // TODO: Parse and validate.
    pub fn from_str(s: &str) -> Self {
        Key(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::NanoClient;

    #[tokio::test]
    async fn real_test() {
        let c = NanoClient::new("10.0.2.169".to_string(), 7076);
        let resp = c.version().await.unwrap();
        dbg!(resp);
    }
}
