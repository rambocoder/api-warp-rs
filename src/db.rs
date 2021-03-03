use mobc_postgres::mobc::Pool;
use mobc_postgres::tokio_postgres::{Config, NoTls};
use mobc_postgres::PgConnectionManager;
use std::str::FromStr;

const DB_POOL_MAX_OPEN: u64 = 20;

pub fn create_pool(
    url: &String,
) -> std::result::Result<Pool<PgConnectionManager<NoTls>>, mobc_postgres::tokio_postgres::Error> {
    println!("{}", url);
    let config = Config::from_str(url);
    let c = match config {
        Ok(conf) => conf,
        Err(err) => return Err(err),
    };
    let manager = PgConnectionManager::new(c, NoTls);
    Ok(Pool::builder().max_open(DB_POOL_MAX_OPEN).build(manager))
}

pub fn create_pool_wild(url: &String) -> Pool<PgConnectionManager<NoTls>> {
    println!("{}", url);
    let config = Config::from_str(url).unwrap();
    let manager = PgConnectionManager::new(config, NoTls);
    Pool::builder().max_open(DB_POOL_MAX_OPEN).build(manager)
}
