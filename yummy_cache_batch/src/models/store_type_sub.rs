use crate::common::*;


#[doc = "MySQL 와 맵핑할 구조체 - store_type_sub 테이블"]
#[derive(Debug, FromQueryResult, Serialize, Setters, new)]
pub struct StoreTypeSubResult {
    pub sub_type: i32,
    pub major_type: i32,
    pub type_name: String,
}