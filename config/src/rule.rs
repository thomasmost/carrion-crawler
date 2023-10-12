use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Orphan {
  pub name: String,
  pub model: String,
  pub parent: String,
  pub model_foreign_key: String,
  pub parent_key: String,
  pub resolution: Resolution,
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Resolution {
  Log,
  Count,
  Delete,
}
