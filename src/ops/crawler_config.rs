use crate::db::establish_connection;
use crate::models::crawler_config::{CrawlerConfig, NewCrawlerConfig};
use crate::schema::crawler_configs;
use diesel::prelude::*;
use diesel::upsert::excluded;

pub fn get_crawler_configs() -> QueryResult<Vec<CrawlerConfig>> {
    let connection = &mut establish_connection();
    crawler_configs::table
        .select(CrawlerConfig::as_select())
        .load(connection)
}

pub fn set_crawler_config(config: NewCrawlerConfig) -> CrawlerConfig {
    let connection = &mut establish_connection();
    diesel::insert_into(crawler_configs::table)
        .values(&config)
        .on_conflict(crawler_configs::crawler_id)
        .do_update()
        .set((
            crawler_configs::interval_seconds.eq(excluded(crawler_configs::interval_seconds)),
            crawler_configs::notice_count.eq(excluded(crawler_configs::notice_count)),
        ))
        .get_result(connection)
        .expect("Error saving crawler configuration")
}

pub fn unset_crawler_config(crawler_id: &str) -> QueryResult<usize> {
    let connection = &mut establish_connection();
    diesel::delete(crawler_configs::table.filter(crawler_configs::crawler_id.eq(crawler_id)))
        .execute(connection)
}
