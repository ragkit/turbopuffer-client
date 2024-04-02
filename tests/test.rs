use serde_json::json;
use std::sync::Once;
use turbopuffer_client::response::{
  self,
  DeleteResponse,
  UpsertResponse,
};

static INIT: Once = Once::new();

fn load_env_once() {
  INIT.call_once(|| {
    dotenv::dotenv().ok();
  });
}

fn api_key() -> String {
  load_env_once();
  std::env::var("TURBOPUFFER_API_KEY")
    .expect("TURBOPUFFER_API_KEY must be set to run tests")
}

#[tokio::test]
pub async fn test_upsert() {
  let client = turbopuffer_client::Client::new(&api_key());
  let ns = client.namespace("test-upsert");
  // Clean up any previous test data.
  let _ = ns.delete().await;

  let vectors = json!({
    "ids": [1, 2, 3, 4],
    "vectors": [[0.1, 0.1], [0.2, 0.2], [0.3, 0.3], [0.4, 0.4]],
    "attributes": {
      "my-string": ["one", null, "three", "four"],
      "my-uint": [12, null, 84, 39],
      "my-string-array": [["a", "b"], ["b", "d"], [], ["c"]]
    }
  });

  let res = ns.upsert(&vectors).await.unwrap();
  assert!(matches!(
    res,
    UpsertResponse {
      status: response::Status::Ok
    }
  ));

  // Now clean it up.
  let res = ns.delete().await.unwrap();
  assert!(matches!(
    res,
    DeleteResponse {
      status: response::Status::Ok
    }
  ));
}

#[tokio::test]
pub async fn test_delete() {
  let client = turbopuffer_client::Client::new(&api_key());
  let ns = client.namespace("test-for-delete");
  // Clean up any previous test data.
  let _ = ns.delete().await;

  let vectors = json!({
    "ids": [1, 2, 3, 4],
    "vectors": [[0.1, 0.1], [0.2, 0.2], [0.3, 0.3], [0.4, 0.4]],
    "attributes": {
      "my-string": ["one", null, "three", "four"],
      "my-uint": [12, null, 84, 39],
      "my-string-array": [["a", "b"], ["b", "d"], [], ["c"]]
    }
  });

  // First create the namespace.
  let res = ns.upsert(&vectors).await.unwrap();
  assert!(matches!(
    res,
    UpsertResponse {
      status: response::Status::Ok
    }
  ));

  // Now delete it.
  let res = ns.delete().await.unwrap();
  assert!(matches!(
    res,
    DeleteResponse {
      status: response::Status::Ok
    }
  ));
}

#[tokio::test]
pub async fn test_query() {
  let client = turbopuffer_client::Client::new(&api_key());
  let ns = client.namespace("test-for-query");
  // Clean up any previous test data.
  let _ = ns.delete().await;

  let vectors = json!({
    "ids": [1, 2, 3, 4],
    "vectors": [[0.1, 0.1], [0.2, 0.2], [0.3, 0.3], [0.4, 0.4]],
    "attributes": {
      "my-string": ["one", null, "three", "four"],
      "my-uint": [12, null, 84, 39],
      "my-string-array": [["a", "b"], ["b", "d"], [], ["c"]]
    }
  });

  // First create the namespace.
  let res = ns.upsert(&vectors).await.unwrap();
  assert!(matches!(
    res,
    UpsertResponse {
      status: response::Status::Ok
    }
  ));

  let query = json!({
    "vector": [0.105, 0.1],
    "distance_metric": "euclidean_squared",
    "top_k": 1,
    "include_vectors": true,
    "include_attributes": ["my-string"],
  });

  // Now run a query.
  let res = ns.query(&query).await.unwrap();
  let vectors = res.vectors;
  assert_eq!(1, vectors.len());
  let first = vectors.first().unwrap();
  assert!(matches!(first.id, response::Id::Int(1)));

  // Now clean it up.
  let res = ns.delete().await.unwrap();
  assert!(matches!(
    res,
    DeleteResponse {
      status: response::Status::Ok
    }
  ));
}
