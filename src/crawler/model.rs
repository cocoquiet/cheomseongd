use chrono::NaiveDate;

/// A notice normalized from a source site, before it is written to the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrawledNotice {
    pub organization_name: String,
    /// Stable identifier assigned by the source site, scoped to the source.
    pub source_key: String,
    pub source_url: String,
    pub title: String,
    pub author: Option<String>,
    pub category: Option<String>,
    pub body_text: Option<String>,
    pub is_pinned: bool,
    pub published_on: Option<NaiveDate>,
}
