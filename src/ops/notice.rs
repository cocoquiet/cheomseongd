use crate::db::establish_connection;
use crate::models::notice::{NewNotice, Notice, UpdateNotice};
use crate::schema::notices;
use diesel::prelude::*;

pub fn create_notice(new_notice: NewNotice) -> Notice {
    let connection = &mut establish_connection();
    diesel::insert_into(notices::table)
        .values(&new_notice)
        .get_result(connection)
        .expect("Error saving new notice")
}

pub fn get_notice_by_id(notice_id: i64) -> QueryResult<Notice> {
    let connection = &mut establish_connection();
    notices::table
        .filter(notices::id.eq(notice_id))
        .select(Notice::as_select())
        .first::<Notice>(connection)
}

pub fn get_notice_by_source(
    organization_name: &str,
    source_key: &str,
) -> QueryResult<Option<Notice>> {
    let connection = &mut establish_connection();
    notices::table
        .filter(notices::organization_name.eq(organization_name))
        .filter(notices::source_key.eq(source_key))
        .select(Notice::as_select())
        .first::<Notice>(connection)
        .optional()
}

pub fn get_notices() -> QueryResult<Vec<Notice>> {
    let connection = &mut establish_connection();
    notices::table
        .select(Notice::as_select())
        .load::<Notice>(connection)
}

pub fn update_notice(notice_id: i64, updated_notice: UpdateNotice) -> QueryResult<Notice> {
    let connection = &mut establish_connection();
    diesel::update(notices::table.filter(notices::id.eq(notice_id)))
        .set(&updated_notice)
        .get_result(connection)
}

pub fn delete_notice(notice_id: i64) -> QueryResult<usize> {
    let connection = &mut establish_connection();
    diesel::delete(notices::table.filter(notices::id.eq(notice_id))).execute(connection)
}
