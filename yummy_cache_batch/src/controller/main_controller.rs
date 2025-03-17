use crate::common::*;

use crate::services::query_service::*;
use crate::services::redis_service::*;

use crate::configuration::cache_schedule_config::*;

use crate::models::store_type_major::*;
use crate::models::store_type_sub::*;

#[derive(Debug, new)]
pub struct MainController<Q: QueryService, R: RedisService> {
    query_service: Q,
    redis_service: R,
}

impl<Q: QueryService, R: RedisService> MainController<Q, R> {
    #[doc = "메인 스케쥴러 함수"]
    /// # Arguments
    /// * `index_schedule` - 인덱스 스케쥴 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_schedule_task(
        &self,
        cache_schedule: CacheScheduleConfig,
    ) -> Result<(), anyhow::Error> {
        // let schedule: Schedule =
        //     Schedule::from_str(&index_schedule.time).expect("Failed to parse CRON expression");

        // let system_config: Arc<SystemConfig> = get_system_config();

        // let mut interval: Interval = tokio::time::interval(tokio::time::Duration::from_millis(
        //     system_config.schedule_term,
        // ));

        // /* 한국 표준시 GMT + 9 */
        // let kst_offset: FixedOffset = match FixedOffset::east_opt(9 * 3600) {
        //     Some(kst_offset) => kst_offset,
        //     None => {
        //         error!(
        //             "[Error][main_schedule_task()] There was a problem initializing 'kst_offset'."
        //         );
        //         panic!(
        //             "[Error][main_schedule_task()] There was a problem initializing 'kst_offset'."
        //         );
        //     }
        // };

        // loop {
        //     interval.tick().await;

        //     let now: DateTime<Utc> = Utc::now();
        //     let kst_now: DateTime<FixedOffset> = now.with_timezone(&kst_offset); /* Converting UTC Current Time to KST */
        //     if let Some(next) = schedule.upcoming(kst_offset).take(1).next() {
        //         if (next - kst_now).num_seconds() < 1 {
        //             match self.main_task(index_schedule.clone()).await {
        //                 Ok(_) => (),
        //                 Err(e) => {
        //                     error!("[Error][main_schedule_task() -> main_task()] {:?}", e);
        //                 }
        //             }
        //         }
        //     }
        // }

        Ok(())
    }

    #[doc = "메인 테스트 함수"]
    /// # Arguments
    /// * `cache_schedule` - 캐시 스케쥴러 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn main_task(
        &self,
        cache_schedule: CacheScheduleConfig,
    ) -> Result<(), anyhow::Error> {
        let function_name: &str = cache_schedule.function_name();

        match function_name {
            "store_type_major_cache" => self.cache_store_type_major(cache_schedule).await?,
            "store_type_sub_cache" => self.cache_store_type_sub(cache_schedule).await?,
            _ => {
                return Err(anyhow!(
                    "[Error][main_task()] The mapped function does not exists.: {}",
                    function_name
                ))
            }
        }

        Ok(())
    }

    #[doc = "store_type_major 테이블의 데이터를 Redis 에 캐시해주는 함수"]
    /// # Arguments
    /// * `cache_schedule` - 캐시 스케쥴러 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn cache_store_type_major(
        &self,
        cache_schedule: CacheScheduleConfig,
    ) -> Result<(), anyhow::Error> {
        
        let cache_key_name: &str = cache_schedule.cache_key_name().as_str();

        /* RDB 에서 store_type_major 테이블의 데이터를 모두 가져와준다. */
        let store_type_majors: Vec<StoreTypeMajorResult> =
            self.query_service.get_all_store_type_major().await?;
        
        /* 레디스에 상점 대분류 캐시를 저장 */
        match self.redis_service.set_key_value(cache_key_name, store_type_majors).await {
            Ok(_) => {
                info!("[cache_store_type_major()] Cache save successful.");
            },
            Err(e) => {
                return Err(anyhow!("[Error][cache_store_type_major()] Failed to save cache.: {:?}", e));
            } 
        }

        Ok(())
    }
    
    #[doc = "store_type_sub 테이블의 데이터를 Redis 에 캐시해주는 함수"]
    /// # Arguments
    /// * `cache_schedule` - 캐시 스케쥴러 객체
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    pub async fn cache_store_type_sub(&self, cache_schedule: CacheScheduleConfig) -> Result<(), anyhow::Error> {

        let cache_key_name: &str = cache_schedule.cache_key_name().as_str();

        /* RDB 에서 store_type_major 테이블의 데이터를 모두 가져와준다. */
        let store_type_subs: Vec<StoreTypeSubResult> =
            self.query_service.get_all_store_type_sub().await?;
        
        let mut major_hash_map: HashMap<i32, Vec<StoreTypeSubResult>> = HashMap::new();

        for store_type in store_type_subs {
            major_hash_map
                .entry(store_type.major_type)
                .or_insert_with(Vec::new)
                .push(store_type);   
        }

        for (major, subs) in &major_hash_map {
            let cache_key_prefix_name: String= format!("{}:{}", cache_key_name, major);

            match self.redis_service.set_key_value(cache_key_prefix_name.as_str(), subs).await {
                Ok(_) => {
                    info!("[cache_store_type_sub()] {} Cache save successful.", cache_key_prefix_name);
                },
                Err(e) => {
                    return Err(anyhow!("[Error][cache_store_type_sub()] Failed to save cache.: {:?}", e));
                } 
            }
        }
        
        Ok(())
    } 

}