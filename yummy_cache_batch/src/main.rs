/*
Author      : Seunghwan Shin
Create date : 2025-03-00
Description :

History     : 2025-03-00 Seunghwan Shin       # [v.1.0.0] first create
*/

mod common;
use common::*;

mod utils_module;
use utils_module::logger_utils::*;

mod controller;

mod entity;

mod models;

mod repository;
use repository::redis_repository::*;

mod services;

#[tokio::main]
async fn main() {
    set_global_logger();
    dotenv().ok();

    info!("Yummy Cache Batch Program Start");

    /* Redis Connection 초기화 */
    init_redis_pool().await;
    
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
}
