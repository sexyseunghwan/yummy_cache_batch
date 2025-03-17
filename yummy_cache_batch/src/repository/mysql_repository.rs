use crate::common::*;

static SQL_DB_POOL: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[doc = "SQL 커넥션 POOL을 초기화 해주는 함수"]
pub async fn establish_connection() -> &'static DatabaseConnection {
    SQL_DB_POOL
        .get_or_init(|| async {
            let db_url: String =
                env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

            let pool_min_cnt: u32 = env::var("SQL_POOL_MIN_CNT")
                .expect("SQL_POOL_MIN_CNT must be set in .env")
                .parse::<u32>()
                .expect("SQL_POOL_MIN_CNT value must be a number.");

            let pool_max_cnt: u32 = env::var("SQL_POOL_MAX_CNT")
                .expect("SQL_POOL_MAX_CNT must be set in .env")
                .parse::<u32>()
                .expect("SQL_POOL_MAX_CNT value must be a number.");

            let timeout_sec: u64 = env::var("SQL_TIMEOUT_SEC")
                .expect("SQL_TIMEOUT_SEC must be set in .env")
                .parse::<u64>()
                .expect("SQL_TIMEOUT_SEC value must be a number.");

            let idle_timeout_sec: u64 = env::var("SQL_IDLE_TIMETOUE_SEC")
                .expect("SQL_IDLE_TIMETOUE_SEC must be set in .env")
                .parse::<u64>()
                .expect("SQL_IDLE_TIMETOUE_SEC value must be a number.");

            let mut opt: ConnectOptions = ConnectOptions::new(db_url);
            opt.max_connections(pool_max_cnt)
                .min_connections(pool_min_cnt)
                .connect_timeout(Duration::from_secs(timeout_sec))
                .idle_timeout(Duration::from_secs(idle_timeout_sec))
                .sqlx_logging(false);

            Database::connect(opt)
                .await
                .expect("Database connection failed")
        })
        .await
}
