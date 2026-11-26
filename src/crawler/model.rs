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
    pub attachments: Vec<CrawledAttachment>,
    pub images: Vec<CrawledImage>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrawledAttachment {
    /// Stable key from the source's attachment link, scoped to this notice.
    pub source_key: String,
    pub file_name: String,
    pub download_url: String,
    pub display_order: i32,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrawledImage {
    pub source_url: String,
    pub alt_text: Option<String>,
    pub display_order: i32,
}
