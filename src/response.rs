use serde::Deserialize;
use serde_json::Value;
use std::{
  collections::HashMap,
  fmt,
};

#[derive(Debug, Deserialize)]
pub enum Status {
  // Necessary because turbopuffer api does not consistently send the same
  // capitalization for "ok". Notably between upsert and delete endpoints.
  #[serde(alias = "ok", alias = "OK")]
  Ok,
}

#[derive(Debug, Deserialize)]
pub struct UpsertResponse {
  pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct DeleteResponse {
  pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct QueryResponse {
  pub vectors: Vec<ResponseVector>,
}

// Required because Ids may be strings or numbers.
// TODO: Consider serializing/deseralizing to only strings to remove this.
// TODO: Consider simply using serde_json::Value.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Id {
  String(String),
  Int(i32),
}

impl fmt::Display for Id {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self {
      Id::String(s) => write!(f, "{}", s),
      Id::Int(i) => write!(f, "{}", i),
    }
  }
}

#[derive(Debug, Deserialize)]
pub struct ResponseVector {
  pub dist: f32,
  pub id: Id,
  // Responses only contain vectors when "include_vectors" is true.
  pub vector: Option<Vec<f32>>,
  pub attributes: Option<HashMap<String, Value>>,
}
