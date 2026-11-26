use crate::schema::images;
use diesel::prelude::*;

use crate::models::notice::Notice;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = images)]
#[diesel(belongs_to(Notice))]
pub struct Image {
    pub id: i64,
    pub notice_id: i64,

    pub source_url: String,
    pub alt_text: Option<String>,
    pub display_order: i32,

    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = images)]
pub struct NewImage<'a> {
    pub notice_id: i64,

    pub source_url: &'a str,
    pub alt_text: Option<&'a str>,
    pub display_order: i32,
}
