use crate::internal_prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct AccountRepresentativeRequest<'a> {
    action: &'a str,
    pub account: &'a Address,
}

impl<'a> AccountRepresentativeRequest<'a> {
    pub fn new(account: &'a Address) -> Self {
        Self {
            action: "account_representative",
            account,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountRepresentativeResponse {
    pub representative: Address,
}

impl NanoClient {
    pub async fn account_representative(&self, request: &AccountRepresentativeRequest<'_>) -> Result<AccountRepresentativeResponse> {
        self.rpc(request).await
    }
}