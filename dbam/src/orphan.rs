use config::rule::Orphan;
use diesel::sql_types::Bigint;
use diesel::{prelude::*, sql_query, MysqlConnection, QueryableByName};

#[derive(Debug, QueryableByName)]
pub struct CountRecord {
  #[diesel(sql_type = Bigint)]
  pub count: i64,
}

pub fn count_orphaned_records(connection: &mut MysqlConnection, rule: &Orphan) -> Result<i64, ()> {
  let Orphan {
    name,
    model,
    parent,
    model_foreign_key,
    parent_key,
    resolution: _r,
  } = rule;
  let query = format!(
        "select count(1) as count from `{model}` c where not exists (select 1 from `{parent}` p where c.{model_foreign_key} = p.{parent_key})");
  let records = sql_query(&query)
    .load::<CountRecord>(connection)
    .expect(&format!("Querying {name} failed; query was `{query}`"));

  Ok(records[0].count)
}
