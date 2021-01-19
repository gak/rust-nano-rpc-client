use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
struct WalletCreateRequest<'a> {
    action: &'a str,
}

impl<'a> WalletCreateRequest<'a> {
    fn new() -> Self {
        Self {
            action: "wallet_create",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct WalletCreateResponse {
    wallet: Wallet,
}

impl NanoClient {
    pub async fn wallet_create(&self) -> Result<WalletCreateResponse> {
        let request = WalletCreateRequest::new();
        self.rpc(&request).await
    }
}