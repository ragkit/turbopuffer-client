use response::UpsertResponse;
use serde_json::Value;

pub mod response;

const BASE_URL: &str = "https://api.turbopuffer.com/v1";

pub struct Client {
  api_key: String,
  client: reqwest::Client,
}

impl Client {
  pub fn new(api_key: &str) -> Self {
    Self {
      api_key: api_key.to_string(),
      client: reqwest::Client::new(),
    }
  }

  pub fn namespace<'a>(&'a self, namespace: &'a str) -> NamespacedClient<'a> {
    NamespacedClient {
      client: self,
      namespace,
    }
  }
}

pub struct NamespacedClient<'a> {
  client: &'a Client,
  namespace: &'a str,
}

impl<'a> NamespacedClient<'a> {
  pub async fn upsert(
    &self,
    vectors: &Value,
  ) -> Result<UpsertResponse, anyhow::Error> {
    let url = format!("{BASE_URL}/vectors/{}", &self.namespace);
    let res = self
      .client
      .client
      .post(url)
      .header("Authorization", format!("Bearer {}", self.client.api_key))
      .header("Content-Type", "application/json")
      .json(vectors)
      .send()
      .await?;

    let value = res.json::<response::UpsertResponse>().await?;
    Ok(value)
  }
}
