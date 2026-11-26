use crate::schema::attachments;
use diesel::prelude::*;

use crate::models::notice::Notice;
use chrono::{DateTime, Utc};

#[derive(Debug, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = attachments)]
#[diesel(belongs_to(Notice))]
pub struct Attachment {
    pub id: i64,
    pub notice_id: i64,

    pub source_key: String,
    pub file_name: String,
    pub download_url: String,
    pub display_order: i32,

    pub content_type: Option<String>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = attachments)]
pub struct NewAttachment<'a> {
    pub notice_id: i64,

    pub source_key: &'a str,
    pub file_name: &'a str,
    pub download_url: &'a str,
    pub display_order: i32,

    pub content_type: Option<&'a str>,
}
