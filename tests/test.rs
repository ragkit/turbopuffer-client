use serde_json::json;
use std::sync::Once;
use turbopuffer_client::response::{
  self,
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
pub async fn test_1() {
  let client = turbopuffer_client::Client::new(&api_key());

  let vectors = json!({
    "ids": [1, 2, 3, 4],
    "vectors": [[0.1, 0.1], [0.2, 0.2], [0.3, 0.3], [0.4, 0.4]],
    "attributes": {
      "my-string": ["one", null, "three", "four"],
      "my-uint": [12, null, 84, 39],
      "my-string-array": [["a", "b"], ["b", "d"], [], ["c"]]
    }
  });

  let res = client.namespace("test").upsert(&vectors).await;
  assert!(matches!(
    res,
    Ok(UpsertResponse {
      status: response::Status::Ok
    })
  ))
}
