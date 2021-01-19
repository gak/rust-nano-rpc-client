use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
pub struct SendRequest<'a> {
    action: &'a str,
    pub wallet: &'a Wallet,
    pub source: &'a Address,
    pub destination: &'a Address,
    pub amount: &'a Raw,
    pub id: &'a str,
}

impl<'a> SendRequest<'a> {
    pub fn new(
        wallet: &'a Wallet,
        source: &'a Address,
        destination: &'a Address,
        amount: &'a Raw,
        id: &'a str,
    ) -> Self {
        Self {
            action: "send",
            wallet,
            source,
            destination,
            amount,
            id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SendResponse {
    block: String,
}

impl NanoClient {
    pub async fn send(&self, request: &SendRequest<'_>) -> Result<SendResponse> {
        self.rpc(request).await
    }
}
