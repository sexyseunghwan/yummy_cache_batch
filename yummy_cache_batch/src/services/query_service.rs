use crate::common::*;

use crate::models::location_city::*;
use crate::models::location_county::*;
use crate::models::location_district::*;
use crate::models::store_type_major::*;
use crate::models::store_type_sub::*;

use crate::repository::mysql_repository::*;

use crate::entity::{
    location_city_tbl, location_county_tbl, location_district_tbl, store_type_major, store_type_sub,
};

pub trait QueryService {
    async fn get_all_store_type_major(&self) -> Result<Vec<StoreTypeMajorResult>, anyhow::Error>;
    async fn get_all_store_type_sub(&self) -> Result<Vec<StoreTypeSubResult>, anyhow::Error>;
    async fn get_all_location_county(&self) -> Result<Vec<LocationCountyResult>, anyhow::Error>;
    async fn get_location_city(
        &self,
        location_county_code: i32,
    ) -> Result<Vec<LocationCityResult>, anyhow::Error>;
    async fn get_location_district(
        &self,
        location_county_code: i32,
        location_city_code: i32,
    ) -> Result<Vec<LocationDistrictResult>, anyhow::Error>;
}

#[derive(Debug, new)]
pub struct QueryServicePub;

impl QueryService for QueryServicePub {
    #[doc = "DB 에서 store_type_major 테이블의 데이터 모두를 가져와주는 함수"]
    async fn get_all_store_type_major(&self) -> Result<Vec<StoreTypeMajorResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let query: Select<store_type_major::Entity> =
            store_type_major::Entity::find().select_only().columns([
                store_type_major::Column::MajorType,
                store_type_major::Column::TypeName,
            ]);

        let store_type_major_results: Vec<StoreTypeMajorResult> =
            query.into_model().all(db).await?;

        Ok(store_type_major_results)
    }

    #[doc = "DB 에서 store_type_sub 테이블의 데이터 모두를 가져와주는 함수"]
    async fn get_all_store_type_sub(&self) -> Result<Vec<StoreTypeSubResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let query: Select<store_type_sub::Entity> = store_type_sub::Entity::find()
            .inner_join(store_type_major::Entity)
            .select_only()
            .columns([
                store_type_sub::Column::SubType,
                store_type_sub::Column::MajorType,
                store_type_sub::Column::TypeName,
            ]);

        let store_type_major_results: Vec<StoreTypeSubResult> = query.into_model().all(db).await?;

        Ok(store_type_major_results)
    }

    #[doc = "DB 에서 location_county 테이블의 데이터 모두를 가져와주는 함수"]
    async fn get_all_location_county(&self) -> Result<Vec<LocationCountyResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let query: Select<location_county_tbl::Entity> =
            location_county_tbl::Entity::find().select_only().columns([
                location_county_tbl::Column::LocationCountyCode,
                location_county_tbl::Column::LocationCounty,
            ]);

        let location_county_results: Vec<LocationCountyResult> = query.into_model().all(db).await?;

        Ok(location_county_results)
    }

    #[doc = "DB 에서 location_city 테이블의 데이터를 가져와주는 함수"]
    async fn get_location_city(
        &self,
        location_county_code: i32,
    ) -> Result<Vec<LocationCityResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let query: Select<location_city_tbl::Entity> = location_city_tbl::Entity::find()
            .inner_join(location_county_tbl::Entity)
            .select_only()
            .columns([
                location_city_tbl::Column::LocationCityCode,
                location_city_tbl::Column::LocationCountyCode,
                location_city_tbl::Column::LocationCity,
            ])
            .filter(location_city_tbl::Column::LocationCountyCode.eq(location_county_code));

        let location_city_results: Vec<LocationCityResult> = query.into_model().all(db).await?;

        Ok(location_city_results)
    }
    
    #[doc = "DB 에서 location_district 테이블의 데이터를 가져와주는 함수"]
    async fn get_location_district(
        &self,
        location_county_code: i32,
        location_city_code: i32,
    ) -> Result<Vec<LocationDistrictResult>, anyhow::Error> {
        let db: &DatabaseConnection = establish_connection().await;

        let query: Select<location_district_tbl::Entity> = location_district_tbl::Entity::find()
            .inner_join(location_city_tbl::Entity)
            .select_only()
            .columns([
                location_district_tbl::Column::LocationDistrictCode,
                location_district_tbl::Column::LocationCityCode,
                location_district_tbl::Column::LocationDistrict,
            ])
            .filter(location_district_tbl::Column::LocationCountyCode.eq(location_county_code))
            .filter(location_district_tbl::Column::LocationCityCode.eq(location_city_code));

        let location_district_results: Vec<LocationDistrictResult> =
            query.into_model().all(db).await?;

        Ok(location_district_results)
    }
}
