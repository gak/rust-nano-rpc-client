use crate::internal_prelude::*;
use crate::Balance;

#[derive(Debug, Serialize)]
pub struct AccountBalanceRequest<'a> {
    action: &'a str,
    pub account: &'a Address,
}

impl<'a> AccountBalanceRequest<'a> {
    pub fn new(account: &'a Address) -> Self {
        Self {
            action: "account_balance",
            account,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    pub balance: Raw,
    pub pending: Raw,
}

impl NanoClient {
    pub async fn account_balance(&self, request: &AccountBalanceRequest<'_>) -> Result<AccountBalanceResponse> {
        self.rpc(request).await
    }
}