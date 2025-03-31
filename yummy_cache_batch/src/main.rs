/*
Author      : Seunghwan Shin
Create date : 2025-03-18
Description : 디비 데이터 캐시 적재 배치 프로그램

History     : 2025-03-18 Seunghwan Shin       # [v.1.0.0] first create
              2025-03-31 Seunghwan Shin       # [v.1.1.0] 음식점 타입 데이터 캐시 적재 추가
*/

mod common;
use common::*;

mod utils_module;
use utils_module::io_utils::*;
use utils_module::logger_utils::*;

mod controller;
use controller::main_controller::*;

mod entity;

mod models;

mod repository;
use repository::redis_repository::*;

mod services;
use services::query_service::*;
use services::redis_service::*;

mod configuration;
use configuration::cache_schedule_config::*;
use configuration::env_config::*;
use configuration::system_config::*;

#[tokio::main]
async fn main() {
    set_global_logger();
    load_env();

    info!("Yummy Cache Batch Program Start");

    let system_infos: Arc<SystemConfig> = get_system_config();
    let compile_type: &str = system_infos.complie_type().as_str();

    /* Redis Connection 초기화 */
    init_redis_pool().await;

    /* Dependency Injection */
    let query_service: QueryServicePub = QueryServicePub::new();
    let redis_service: RedisServicePub = RedisServicePub::new();
    let controller_arc: Arc<MainController<QueryServicePub, RedisServicePub>> =
        Arc::new(MainController::new(query_service, redis_service));

    let cache_schedules: CacheScheduleConfigList =
        match read_toml_from_file::<CacheScheduleConfigList>(&CACHE_LIST_PATH) {
            Ok(cache_schedules) => cache_schedules,
            Err(e) => {
                error!("[Error][main()] {:?}", e);
                panic!("[Error][main()] {:?}", e);
            }
        };

    if compile_type == "schedule" {
        /*
            [스케쥴 타입의 캐시배치 프로그램]
            각 데이터별로 캐싱배치를 비동기적으로 실시해준다.
            스케쥴링 대기 작업 진행
        */
        for cache in cache_schedules.cache {
            let cache_clone: CacheScheduleConfig = cache.clone();
            let controller_arc_clone: Arc<MainController<QueryServicePub, RedisServicePub>> = Arc::clone(&controller_arc);

            tokio::spawn(async move {
                if let Err(e) = controller_arc_clone.main_schedule_task(cache_clone).await {
                    error!("[Error][main_schedule_task] {:?}", e);
                }
            });
        }

        /* 모두 서브테스크로 실행되므로 아래와 같이 메인 테스크를 계속 유지시켜줘야 한다. */
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Received Ctrl+C, shutting down...");
            }
        }

    } else if compile_type == "cli" {

        /* 사용자 입력을 받아서 캐싱을 처리하는 프로그램 */
        match controller_arc.cli_cache_task(cache_schedules).await {
            Ok(_) => (),
            Err(e) => {
                error!("[Error][main()] {:?}", e);
                panic!("[Error][main()] {:?}", e);
            }
        }

    } else {
        error!("[Error][main()] The 'COMPILE_TYPE' information must be 'schedule' or 'cli'.");
        panic!("[Error][main()] The 'COMPILE_TYPE' information must be 'schedule' or 'cli'.");
    }

    /* TEST CODE */
    // let cache_schedule: &CacheScheduleConfig = cache_schedules.cache().get(0).unwrap();
    // controller_arc
    //     .main_task(cache_schedule.clone())
    //     .await
    //     .unwrap();
}

/* Redis Connection 초기화 */
//init_redis_pool().await;

// let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();

// for i in 0..20 {
//     let handle = tokio::spawn(async move {
//         let mut conn: ClusterConnection = get_redis_conn().await.unwrap();
//         let pong: String = redis::cmd("PING")
//             .query_async(&mut conn)
//             .await
//             .expect("PING 명령 실패");

//         println!("Task {}: PING response: {}", i, pong);
//         return_redis_conn(conn).await;

//         // 약간의 지연을 주어 태스크 간에 경쟁 조건이 발생하도록 함
//         //tokio::time::sleep(Duration::from_millis(50 * i)).await;
//         //let conn = get_connection()
//         // if let Some(mut conn) = get_connection().await {
//         //     let pong: String = redis::cmd("PING")
//         //         .query_async(&mut conn)
//         //         .await
//         //         .expect("PING 명령 실패");
//         //     println!("Task {}: PING response: {}", i, pong);
//         //     return_connection(conn).await;
//         // } else {
//         //     println!("Task {}: 사용 가능한 커넥션 없음", i);
//         // }
//     });
//     handles.push(handle);
// }

// 모든 태스크가 완료될 때까지 대기
// for handle in handles {
//     handle.await.unwrap();
// }
