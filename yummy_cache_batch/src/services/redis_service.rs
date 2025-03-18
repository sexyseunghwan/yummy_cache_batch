use crate::common::*;

use crate::repository::redis_repository::*;

#[async_trait]
pub trait RedisService {
    async fn set_key_value<T: Serialize + Send>(&self, key_name: &str, value: T) -> Result<(), anyhow::Error>;
}

#[derive(Debug, new)]
pub struct RedisServicePub;

#[async_trait]
impl RedisService for RedisServicePub {


    #[doc = "레디스에 키-값 데이터를 저장해주는 함수"]
    /// # Arguments
    /// * `key_name` - 레디스에 저장할 키 이름
    /// * `value` - 레디스 키에 저장할 value
    /// 
    /// # Returns
    /// * Result<(), anyhow::Error>
    async fn set_key_value<T: Serialize + Send>(&self, key_name: &str, value: T) -> Result<(), anyhow::Error> {

        let json_value: String = serde_json::to_string(&value)?;

        let mut redis_conn: ClusterConnection = get_redis_conn().await?;
        
        match redis_conn.set::<&str, String, ()>(key_name, json_value).await {
            Ok(_) => {
                return_redis_conn(redis_conn).await; /* 필수 -> Redis connection 반납. */
                Ok(())
            },
            Err(e) => {
                return_redis_conn(redis_conn).await; /* 필수 -> Redis connection 반납. */
                return Err(anyhow!("{:?}", e))
            }
        }
    }
}   
