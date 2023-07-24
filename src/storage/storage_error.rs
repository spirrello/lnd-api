#[derive(Debug)]
pub struct RedisError {
    pub msg: String,
}
impl RedisError {
    pub fn new_str(s: &str) -> RedisError {
        RedisError { msg: s.to_string() }
    }
    pub fn new_string(s: String) -> RedisError {
        RedisError { msg: s }
    }
}
