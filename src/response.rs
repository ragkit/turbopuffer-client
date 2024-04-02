use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
  Ok,
}

#[derive(Deserialize)]
pub struct UpsertResponse {
  pub status: Status,
}
