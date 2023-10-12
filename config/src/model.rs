use serde_derive::Deserialize;

use crate::rule::Orphan;

#[derive(Deserialize)]
pub struct Data {
  pub database: DatabaseConfig,
  pub rules: Rules,
}

// Config struct holds to data from the `[database]` section.
#[derive(Deserialize)]
pub struct DatabaseConfig {
  pub database_url: String,
}

// Config struct holds to data from the `[rules]` section.
#[derive(Deserialize)]
pub struct Rules {
  pub orphans: Vec<Orphan>,
}
