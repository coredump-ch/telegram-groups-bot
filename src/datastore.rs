//! Data storage backend.
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;
