use crate::internal_prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct WalletBalancesRequest<'a> {
    action: &'a str,
    pub wallet: &'a Wallet,
}

impl<'a> WalletBalancesRequest<'a> {
    pub fn new(wallet: &'a Wallet) -> Self {
        Self {
            action: "wallet_balances",
            wallet,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    balance: Raw,
    pending: Raw,
}

#[derive(Debug, Deserialize)]
pub struct WalletBalancesResponse {
    balances: HashMap<Address, Balance>
}

impl NanoClient {
    pub async fn wallet_balances(&self, request: &WalletBalancesRequest<'_>) -> Result<WalletBalancesResponse> {
        self.rpc(request).await
    }
}