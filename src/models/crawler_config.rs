use crate::schema::crawler_configs;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crawler_configs)]
#[diesel(primary_key(crawler_id))]
pub struct CrawlerConfig {
    pub crawler_id: String,
    pub interval_seconds: i64,
    pub notice_count: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crawler_configs)]
pub struct NewCrawlerConfig<'a> {
    pub crawler_id: &'a str,
    pub interval_seconds: i64,
    pub notice_count: i32,
}
