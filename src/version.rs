use crate::internal_prelude::*;

#[derive(Debug, Serialize)]
struct VersionRequest<'a> {
    action: &'a str,
}

impl VersionRequest<'_> {
    fn new() -> Self {
        VersionRequest { action: "version" }
    }
}

#[derive(Debug, Deserialize)]
pub struct VersionResponse {
    node_vendor: String,
}

impl NanoClient {
    pub async fn version(&self) -> Result<VersionResponse> {
        let request = VersionRequest::new();
        self.rpc(&request).await
    }
}