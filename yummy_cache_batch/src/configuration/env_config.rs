use crate::common::*;

#[doc = "Function to globally initialize the 'INDEX_LIST_PATH' variable"]
pub static CACHE_LIST_PATH: once_lazy<String> = once_lazy::new(|| {
    dotenv().ok();
    env::var("INDEX_LIST_PATH").expect("[ENV file read Error] 'INDEX_LIST_PATH' must be set")
});