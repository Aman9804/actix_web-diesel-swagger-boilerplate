/* This file is generated and managed by xynes */

use crate::diesel::*;
use crate::schema::*;
use paperclip::actix::Apiv2Schema;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use diesel_async::RunQueryDsl;


type Connection = diesel_async::AsyncPgConnection;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Apiv2Schema, Insertable, AsChangeset, Selectable)]
#[diesel(table_name=users, primary_key(id))]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub sex: String,
    pub mobile: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Apiv2Schema, Insertable, AsChangeset)]
#[diesel(table_name=users)]
pub struct CreateUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub sex: String,
    pub mobile: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Apiv2Schema, Insertable, AsChangeset)]
#[diesel(table_name=users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub sex: Option<String>,
    pub mobile: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}


#[derive(Debug, Serialize, Apiv2Schema)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_items: i64,
    /// 0-based index
    pub page: i64,
    pub page_size: i64,
    pub num_pages: i64,
}

impl User {

    pub async fn create(db: &mut Connection, item: &CreateUser) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        insert_into(users).values(item).get_result::<Self>(db).await
    }

    pub async fn read(db: &mut Connection, param_id: uuid::Uuid) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        users.filter(id.eq(param_id)).first::<Self>(db).await
    }

    /// Paginates through the table where page is a 0-based index (i.e. page 0 is the first page)
    pub async fn paginate(db: &mut Connection, page: i64, page_size: i64) -> QueryResult<PaginationResult<Self>> {
        use crate::schema::users::dsl::*;

        let page_size = if page_size < 1 { 1 } else { page_size };
        let total_items = users.count().get_result(db).await?;
        let items = users.limit(page_size).offset(page * page_size).load::<Self>(db).await?;

        Ok(PaginationResult {
            items,
            total_items,
            page,
            page_size,
            /* ceiling division of integers */
            num_pages: total_items / page_size + i64::from(total_items % page_size != 0)
        })
    }

    pub async fn update(db: &mut Connection, param_id: uuid::Uuid, item: &UpdateUser) -> QueryResult<Self> {
        use crate::schema::users::dsl::*;

        diesel::update(users.filter(id.eq(param_id))).set(item).get_result(db).await
    }

    pub async fn delete(db: &mut Connection, param_id: uuid::Uuid) -> QueryResult<usize> {
        use crate::schema::users::dsl::*;

        diesel::delete(users.filter(id.eq(param_id))).execute(db).await
    }

}