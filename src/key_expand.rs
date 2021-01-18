use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
pub struct KeyExpandRequest<'a> {
    action: &'a str,
    pub key: &'a Key,
}

impl<'a> KeyExpandRequest<'a> {
    pub fn new(key: &'a Key) -> Self {
        Self {
            action: "key_expand",
            key,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct KeyExpandResponse {
    private: Key,
    public: Key,
    account: Address,
}

impl NanoClient {
    pub async fn key_expand(&self, request: &KeyExpandRequest<'_>) -> Result<KeyExpandResponse> {
        self.rpc(request).await
    }
}