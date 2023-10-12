use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use std::env;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Pool {
  let manager = ConnectionManager::<MysqlConnection>::new(database_url());
  let pool = Pool::builder()
    .max_size(32)
    .min_idle(Some(8))
    .build(manager)
    .expect("Database Pool failed to initialize");
  println!(
    "Initialized db pool with min_idle {:?} and max_size {}",
    pool.min_idle(),
    pool.max_size()
  );
  pool
}

fn database_url() -> String {
  env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
