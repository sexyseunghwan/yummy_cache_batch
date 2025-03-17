use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체 - store_type_major 테이블"]
#[derive(Debug, FromQueryResult, Serialize, Setters, new)]
pub struct StoreTypeMajorResult {
    pub major_type: i32,
    pub type_name: String,
}

// #[doc = "음식점 대분류 구조체 - Redis와 맵핑"]
// #[derive(Debug, Serialize, Setters, new)]
// #[getset(get = "pub", set = "pub")]
// pub struct StoreTypeMajor {
//     pub major_type: i32,
//     pub type_name: String,
// }
