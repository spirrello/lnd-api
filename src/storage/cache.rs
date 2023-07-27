use redis::FromRedisValue;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use deadpool_redis::{redis::cmd, Config, Connection, Pool, Runtime};
use redis::AsyncCommands;
use std::env;

use super::storage_error::RedisError;

pub type DeadpoolPool = Pool;
type DeadpoolConnection = Connection;

const PREFIX: &str = "with_deadpool";
const TTL: usize = 60 * 5;
const MAX_POOL_SIZE: usize = 30;
const WAIT_TIMEOUT: Option<Duration> = Some(Duration::from_secs(10));

#[derive(Serialize, Deserialize)]
pub struct CachePayload {
    hash_key: String,
    values: Vec<(String, String)>,
}

impl CachePayload {
    pub fn new(hash_key: String, values: Vec<(String, String)>) -> CachePayload {
        CachePayload { hash_key, values }
    }
}

/// Creates connection pool with default settings
pub fn _simple_create_pool(host_addr: &str) -> Result<DeadpoolPool, RedisError> {
    let config = Config::from_url(host_addr);
    config
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|e| RedisError::new_string(e.to_string()))
}

pub fn create_pool() -> Result<DeadpoolPool, RedisError> {
    let redis_addr =
        env::var("REDIS_HOSTNAME").expect("missing environment variable REDIS_HOSTNAME");
    let uri_scheme = match env::var("IS_TLS") {
        Ok(_) => "rediss",
        Err(_) => "redis",
    };
    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();
    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_addr);

    let config = Config::from_url(redis_conn_url);
    config
        .builder()
        .map(|b| {
            b.max_size(MAX_POOL_SIZE)
                .wait_timeout(WAIT_TIMEOUT) // TODO needs create_timeout/recycle timeout?
                .runtime(Runtime::Tokio1)
                .build()
                .unwrap() // TODO don't panic. flat_map can't be used???
        })
        .map_err(|e| RedisError::new_string(e.to_string()))
}

pub async fn create_connection(pool: &DeadpoolPool) -> Result<DeadpoolConnection, RedisError> {
    pool.get()
        .await
        .map_err(|e| RedisError::new_string(e.to_string()))
}

pub async fn set_hash(
    pool: &DeadpoolPool,
    cache_payload: CachePayload, // values: &'a [(String, String)],
) -> Result<(), RedisError> {
    let mut conn = create_connection(pool).await?;

    let mut hash_values = Vec::new();
    for x in &cache_payload.values {
        hash_values.push((x.0.as_str(), x.1.as_str()));
    }

    cmd("HMSET")
        .arg(cache_payload.hash_key)
        .arg(&hash_values[..])
        .query_async::<_, ()>(&mut conn)
        .await
        .unwrap();

    Ok(())
}
