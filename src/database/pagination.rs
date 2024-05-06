use serde::Deserialize;
use sqlx::{Execute, MySql, QueryBuilder};

#[derive(Deserialize)]
pub struct PaginationOption {
    limit: Option<u32>,
    page: Option<u32>,
}

pub fn paginate(query: &str, option: PaginationOption) -> String {
    let page = option.page.unwrap_or(1);
    let limit = option.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut query_builder = QueryBuilder::<MySql>::new(query);

    query_builder.push(" LIMIT ");
    query_builder.push_bind(limit);
    query_builder.push(" OFFSET ");
    query_builder.push_bind(offset);

    query_builder.build().sql().into()
}
