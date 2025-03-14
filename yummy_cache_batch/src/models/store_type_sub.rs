use crate::common::*;

#[doc = "음식점 t소분류 구조체"]
#[derive(Debug, Serialize, Setters, new)]
#[getset(get = "pub", set = "pub")]
pub struct StoreTypeSub {
    pub sub_type: i32,
    pub major_type: i32,
    pub type_name: String
}