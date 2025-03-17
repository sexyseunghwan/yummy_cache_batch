use crate::common::*;

pub type RedisClusterConnectionPool = Arc<Mutex<Vec<ClusterConnection>>>;

#[doc = "전역적으로 사용할 Redis 클러스터 풀 (싱글톤)"]
pub static REDIS_POOL: OnceCell<RedisClusterConnectionPool> = OnceCell::const_new();

#[doc = "Redis connection pool 을 초기화 시켜주는 함수"]
pub async fn init_redis_pool() {
    let redis_nodes =
        env::var("REDIS_NODES").expect("REDIS_NODES environment variable is required.");
    let pool_size: usize = env::var("POOL_SIZE")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .expect("POOL_SIZE must be a number.");
    
    let nodes: Vec<String> = redis_nodes
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let client: ClusterClient =
        ClusterClient::new(nodes).expect("Redis Cluster Client Creation Failed");

    let mut connections: Vec<ClusterConnection> = Vec::with_capacity(pool_size);

    /* 비동기적으로 커넥션을 여러 개 생성하여 풀을 구성 */
    for _ in 0..pool_size {
        let conn = client
            .get_async_connection()
            .await
            .expect("Redis cluster connection failed");
        connections.push(conn);
    }

    let pool: Arc<Mutex<Vec<ClusterConnection>>> = Arc::new(Mutex::new(connections));

    if REDIS_POOL.set(pool).is_err() {
        panic!("REDIS_POOL initialization failed");
    }
}

#[doc = "Redis connection 을 반환하는 함수"]
pub async fn get_redis_conn() -> Result<ClusterConnection, anyhow::Error> {
    for try_cnt in 1..=10 {
        let redis_repo: Option<ClusterConnection> = {
            let pool: Option<ClusterConnection> = match REDIS_POOL.get() {
                Some(pool) => {
                    let mut guard: MutexGuard<'_, Vec<ClusterConnection>> = pool.lock().await;
                    guard.pop()
                }
                None => {
                    panic!("[Error][get_connection()] The value 'REDIS_POOL' does not exist.");
                }
            };

            pool
        };

        if let Some(redis_repo) = redis_repo {
            return Ok(redis_repo);
        }

        warn!(
            "[Attempt {}] The Redis connection pool does not have an idle connection.",
            try_cnt
        );

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    return Err(anyhow!(
        "[Error][get_connection()] Cannot Find Redis Connection"
    ));
}

#[doc = "Redis connection 을 반납하는 함수"]
pub async fn return_redis_conn(conn: ClusterConnection) {
    if let Some(pool) = REDIS_POOL.get() {
        let mut guard: MutexGuard<'_, Vec<ClusterConnection>> = pool.lock().await;
        guard.push(conn);
        info!("[return] redis connection: {}", guard.len());
    }
}
