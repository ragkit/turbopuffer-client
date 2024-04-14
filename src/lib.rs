use error::Error;
use response::{
  DeleteResponse,
  QueryResponse,
  ResponseVector,
  UpsertResponse,
};
use serde_json::{
  from_value,
  Value,
};

pub mod error;
pub mod response;

const BASE_URL: &str = "https://api.turbopuffer.com/v1";

#[derive(Clone)]
pub struct Client {
  api_key: String,
  client: reqwest::Client,
}

impl Client {
  /// Create a new client with the given API key.
  ///
  /// Example:
  ///
  /// ```
  /// use turbopuffer_client::Client;
  ///
  /// let api_key = "secret";
  /// let client = Client::new(api_key);
  /// ```
  ///
  /// Panics: This method panics if a TLS backend cannot be initialized, or the
  /// resolver cannot load the system configuration.
  pub fn new(api_key: &str) -> Self {
    Self {
      api_key: api_key.to_string(),
      client: reqwest::Client::new(),
    }
  }

  /// Scope the client to a namespace. All following operations will run on
  /// this namespace.
  ///
  /// Example:
  ///
  /// ```
  /// use turbopuffer_client::Client;
  ///
  /// let ns = Client::new("secret").namespace("test");
  /// ```
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
  /// Upsert a vector into a namespace. This creates the namespace if it does
  /// not yet have any vectors.
  ///
  /// Example:
  ///
  /// ```no_run
  /// use turbopuffer_client::Client;
  ///
  /// let ns = Client::new("secret").namespace("test");
  ///
  /// let vectors = json!({
  ///   "ids": [1, 2, 3, 4],
  ///   "vectors": [[0.1, 0.1], [0.2, 0.2], [0.3, 0.3], [0.4, 0.4]],
  ///   "attributes": {
  ///     "my-string": ["one", null, "three", "four"],
  ///     "my-uint": [12, null, 84, 39],
  ///     "my-string-array": [["a", "b"], ["b", "d"], [], ["c"]]
  ///   }
  /// });
  ///
  /// let res = ns.upsert(&vectors).await.unwrap();
  /// ```
  pub async fn upsert(&self, body: &Value) -> Result<UpsertResponse, Error> {
    let url = format!("{BASE_URL}/vectors/{}", &self.namespace);
    let res = self
      .client
      .client
      .post(url)
      .header("Authorization", format!("Bearer {}", self.client.api_key))
      .header("Content-Type", "application/json")
      .json(body)
      .send()
      .await?;

    let value = res.json::<Value>().await.map_err(error::non_json)?;

    // TODO: Remove or defer clone.
    from_value::<UpsertResponse>(value.clone())
      .map_err(|e| error::invalid_response(e, value))
  }

  /// Query the namespace for matching vectors.
  ///
  /// Example:
  ///
  /// ```no_run
  /// use turbopuffer_client::Client;
  ///
  /// let ns = Client::new("secret").namespace("test");
  ///
  /// let query = json!({
  ///   "vector": [0.105, 0.1],
  ///   "distance_metric": "euclidean_squared",
  ///   "top_k": 1,
  ///   "include_vectors": true,
  ///   "include_attributes": ["my-string"],
  /// });
  ///
  /// let res = ns.query(&query).await.unwrap();
  /// ```
  pub async fn query(&self, body: &Value) -> Result<QueryResponse, Error> {
    let url = format!("{BASE_URL}/vectors/{}/query", &self.namespace);
    let res = self
      .client
      .client
      .post(url)
      .header("Authorization", format!("Bearer {}", self.client.api_key))
      .header("Content-Type", "application/json")
      .json(body)
      .send()
      .await?;

    let value = res.json::<Value>().await.map_err(error::non_json)?;
    // TODO: Remove or defer clone.
    let vectors = from_value::<Vec<ResponseVector>>(value.clone())
      .map_err(|e| error::invalid_response(e, value))?;
    Ok(QueryResponse { vectors })
  }

  /// Deletes the namespace and all related data.
  ///
  /// Example:
  ///
  /// ```ignore
  /// let res = ns.delete().await.unwrap();
  /// ```
  pub async fn delete(&self) -> Result<DeleteResponse, Error> {
    let url = format!("{BASE_URL}/vectors/{}", &self.namespace);
    let res = self
      .client
      .client
      .delete(url)
      .header("Authorization", format!("Bearer {}", self.client.api_key))
      .header("Content-Type", "application/json")
      .send()
      .await?;

    let value = res.json::<Value>().await.map_err(error::non_json)?;
    // TODO: Remove or defer clone.
    from_value::<DeleteResponse>(value.clone())
      .map_err(|e| error::invalid_response(e, value))
  }
}
