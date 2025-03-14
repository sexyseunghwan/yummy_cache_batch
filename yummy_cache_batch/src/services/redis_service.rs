use crate::common::*;


pub trait RedisService {}

#[derive(Debug, new)]
pub struct RedisServicePub;

impl RedisService for RedisServicePub {
    
}