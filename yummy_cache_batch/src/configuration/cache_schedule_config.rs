use crate::common::*;

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct CacheScheduleConfig {
    pub cache_name: String,
    pub cache_key_name: String,
    pub time: String,
    pub function_name: String,
    pub sql_batch_size: usize
}

#[derive(Debug, Deserialize, Serialize, Getters, Clone)]
#[getset(get = "pub")]
pub struct CacheScheduleConfigList {
    pub cache: Vec<CacheScheduleConfig>
}