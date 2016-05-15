//! Data storage backend.
use url::Url;
use redis::{Commands, RedisError};
use r2d2::{Pool, GetTimeout};
use r2d2_redis::RedisConnectionManager;

const hash_key: &'static str = "topicgroups";

pub type RedisPool = Pool<RedisConnectionManager>;

quick_error! {
    #[derive(Debug)]
    pub enum DatastoreError {
        /// Redis error
        Redis(err: RedisError) {
            from()
            cause(err)
        }
        /// R2D2 pool error
        PoolTimeout(err: GetTimeout) {
            from()
            cause(err)
        }
    }
}

/// Convert a group id into a hash key used for storing and retrieving data.
///
/// This is done to get some additional namespacing in case the database is
/// shared.
fn get_hash_key(group_id: i64) -> String {
    format!("{}:{}", hash_key, group_id)
}

/// Save a topic group to Redis.
///
/// Args:
/// - group_id: The id of the main group.
/// - topic: The name of the topic group.
/// - url: The group url.
/// - pool: A RedisPool instance.
pub fn save_group(group_id: i64, topic: &str, url: &Url, pool: RedisPool)
                  -> Result<(), DatastoreError> {
    // Connect to Redis
    let conn = try!(pool.get());

    // Store value
    try!(conn.hset(get_hash_key(group_id), topic, url.as_str()));

    Ok(())
}
