use thiserror::Error;
use serde::{Serialize, Deserialize};

struct Nano {
    address: String,
    port: u16,
}

#[derive(Error, Debug)]
pub enum NanoError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },

    #[error("request error")]
    RequestError(#[from] reqwest::Error),

    #[error("Unknown nano error")]
    Unknown,
}

type Result<T> = std::result::Result<T, NanoError>;

impl Nano {
    fn new(address: String, port: u16) -> Self {
        Self {
            address,
            port,
        }
    }

    async fn version(&self) -> Result<VersionResponse> {
        let req = VersionRequest { action: "version".to_string() };

        let client = reqwest::Client::new();
        let res = client.post(&format!("http://{}:{}/", self.address, self.port))
            .json(&req)
            .send()
            .await?;

        Ok(res.json().await?)
    }
}

#[derive(Serialize)]
struct VersionRequest {
    action: String,
}

#[derive(Debug, Deserialize)]
struct VersionResponse {
    node_vendor: String,
}


#[cfg(test)]
mod tests {
    use crate::Nano;

    #[tokio::test]
    async fn real_test() {
        let c = Nano::new("10.0.2.169".to_string(), 7076);
        let resp = c.version().await.unwrap();
        dbg!(resp);
    }
}
