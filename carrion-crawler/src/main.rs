use std::env;

use config::rule::Resolution;
use dbam::db::init_pool;

fn main() {
  let data = config::read_config();
  env::set_var("DATABASE_URL", data.database.database_url);
  let count_orphan_rules = data.rules.orphans.len();
  let count_rules = count_orphan_rules;
  println!("Initialized with {} rules", count_rules);
  let pool = init_pool();

  let connection = &mut pool.get().unwrap();
  for orphan_rule in data.rules.orphans {
    if orphan_rule.resolution == Resolution::Count {
      let count = dbam::orphan::count_orphaned_records(connection, &orphan_rule).unwrap();
      println!("{}: {}", orphan_rule.name, count);
    }
  }
  println!("All rules completed successfully");
}
