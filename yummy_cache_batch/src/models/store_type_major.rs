use crate::common::*;

#[doc = "음식점 대분류 구조체"]
#[derive(Debug, Serialize, Setters, new)]
#[getset(get = "pub", set = "pub")]
pub struct StoreTypeMajor {
    pub major_type: i32,
    pub type_name: String
}