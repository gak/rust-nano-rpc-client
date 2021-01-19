use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
pub struct ReceiveMinimumSetRequest<'a> {
    action: &'a str,
    pub amount: &'a Raw,
}

impl<'a> ReceiveMinimumSetRequest<'a> {
    pub fn new(amount: &'a Raw) -> Self {
        Self {
            action: "receive_minimum_set",
            amount,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReceiveMinimumSetResponse {}

impl NanoClient {
    pub async fn receive_minimum_set(&self, request: &ReceiveMinimumSetRequest<'_>) -> Result<ReceiveMinimumSetResponse> {
        self.rpc(request).await
    }
}