use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
pub struct AccountWeightRequest<'a> {
    action: &'a str,
    pub account: &'a Address,
}

impl<'a> AccountWeightRequest<'a> {
    pub fn new(account: &'a Address) -> Self {
        Self {
            action: "account_weight",
            account,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountWeightResponse {
    pub weight: Raw,
}

impl NanoClient {
    pub async fn account_weight(&self, request: &AccountWeightRequest<'_>) -> Result<AccountWeightResponse> {
        self.rpc(request).await
    }
}