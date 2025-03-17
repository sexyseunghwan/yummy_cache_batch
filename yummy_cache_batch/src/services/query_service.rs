use crate::common::*;

use crate::models::store_type_major::*;
use crate::models::store_type_sub::*;

use crate::repository::mysql_repository::*;

use crate::entity::{store_type_major, store_type_sub};

pub trait QueryService {
    async fn get_all_store_type_major(&self) -> Result<Vec<StoreTypeMajorResult>, anyhow::Error>;
    async fn get_all_store_type_sub(&self) -> Result<Vec<StoreTypeSubResult>, anyhow::Error>;
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
                store_type_sub::Column::TypeName
            ]);

        let store_type_major_results: Vec<StoreTypeSubResult> =
            query.into_model().all(db).await?;

        Ok(store_type_major_results)
    }
}
