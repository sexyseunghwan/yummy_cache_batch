use crate::common::*;

#[doc = "MySQL 와 맵핑할 구조체 - location_city_tbl 테이블"]
#[derive(Debug, FromQueryResult, Serialize, Setters, new)]
pub struct LocationCityResult {
    pub location_city_code: i32,
    pub location_county_code: i32,
    pub location_city: String,
}
