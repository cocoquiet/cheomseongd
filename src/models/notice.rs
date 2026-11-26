use crate::schema::notices;
use diesel::prelude::*;

use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = notices)]
pub struct Notice {
    pub id: i64,

    pub organization_name: String,
    pub source_key: String,
    pub source_url: String,

    pub title: String,
    pub author: Option<String>,
    pub category: Option<String>,
    pub body_text: Option<String>,
    pub is_pinned: bool,
    pub published_on: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notices)]
pub struct NewNotice<'a> {
    pub organization_name: &'a str,
    pub source_key: &'a str,
    pub source_url: &'a str,

    pub title: &'a str,
    pub author: Option<&'a str>,
    pub category: Option<&'a str>,
    pub body_text: Option<&'a str>,
    pub is_pinned: bool,
    pub published_on: Option<NaiveDate>,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = notices)]
pub struct UpdateNotice<'a> {
    pub organization_name: Option<&'a str>,
    pub source_key: Option<&'a str>,
    pub source_url: Option<&'a str>,

    pub title: Option<&'a str>,
    pub author: Option<&'a str>,
    pub category: Option<&'a str>,
    pub body_text: Option<&'a str>,
    pub is_pinned: Option<bool>,
    pub published_on: Option<NaiveDate>,
}
