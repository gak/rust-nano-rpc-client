use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
pub struct WalletAddRequest<'a> {
    action: &'a str,
    pub wallet: &'a Wallet,
    pub key: &'a Key,
}

impl<'a> WalletAddRequest<'a> {
    pub fn new(wallet: &'a Wallet, key: &'a Key) -> Self {
        Self {
            action: "wallet_add",
            wallet,
            key,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WalletAddResponse {
    account: Address,
}

impl NanoClient {
    pub async fn wallet_add(&self, request: &WalletAddRequest<'_>) -> Result<WalletAddResponse> {
        self.rpc(request).await
    }
}