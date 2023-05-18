// Source: https://github.com/diesel-rs/diesel/blob/master/examples/postgres/advanced-blog-cli/src/pagination.rs


use diesel::pg::Pg;
// use diesel::prelude::*;
use super::page::Page;
use diesel::sql_types::BigInt;
use diesel::{query_builder::*, QueryResult};
use diesel_async::methods::LoadQuery;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub trait SortingAndPaging: Sized {
    fn paginate(self, page: i64) -> SortedAndPaginated<Self>;
}

impl<T> SortingAndPaging for T {
    fn paginate(self, page: i64) -> SortedAndPaginated<Self> {
        SortedAndPaginated {
            query: self,
            sort_by: crate::constants::EMPTY_STR.to_string(),
            sort_direction: crate::constants::EMPTY_STR.to_string(),
            per_page: crate::constants::DEFAULT_PER_PAGE,
            page,
            offset: 0
        }
    }
}

#[derive(Debug, Clone, QueryId)]
pub struct SortedAndPaginated<T> {
    query: T,
    sort_by: String,
    sort_direction: String,
    page: i64,
    per_page: i64,
    offset:i64,
}

impl<T> SortedAndPaginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        SortedAndPaginated { per_page,offset:(self.page-1)*per_page, ..self }
    }

    pub fn sort(self, sort_by: String, sort_direction: String) -> Self {
        SortedAndPaginated {
            sort_by,
            sort_direction,
            ..self
        }
    }

    pub async fn load_and_count_items<U>(
        self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> QueryResult<Page<U>>
    where
        for<'a> Self: LoadQuery<'a, AsyncPgConnection, (U, i64)>,
        U: Send,
    {
        let page = self.page;
        let per_page = self.per_page;
        let results = self.load::<(U, i64)>(conn).await?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let number_of_pages = total / per_page + i64::from(total % per_page != 0);
        Ok(Page::new(number_of_pages, records, page, per_page, total))
    }
}

impl<T: Query> Query for SortedAndPaginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

// impl<T> RunQueryDsl<diesel_async::AsyncPgConnection> for SortedAndPaginated<T> {}

impl<T> QueryFragment<Pg> for SortedAndPaginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()>
    {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t ");
        if &self.sort_by.as_str().len() > &0 {
            out.push_sql(format!(" ORDER BY {} {}", &self.sort_by, &self.sort_direction).as_str());
        }
        out.push_sql(" LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}
