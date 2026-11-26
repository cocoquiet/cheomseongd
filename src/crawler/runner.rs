use super::{CrawlError, Crawler, CrawledNotice};
use crate::models::notice::{NewNotice, Notice};
use crate::ops::notice::create_notice;

/// Runs one crawler and returns its normalized notices.
///
/// Keeping persistence outside this function lets the runner report source
/// failures without coupling crawler implementations to Diesel models.
pub async fn crawl<C: Crawler + ?Sized>(
    crawler: &C,
) -> Result<Vec<CrawledNotice>, CrawlError> {
    crawler.crawl().await
}

/// Crawls one source and inserts the returned notices through the existing DB operation.
///
/// Database errors follow the current `create_notice` behavior, which panics on failure.
pub async fn run<C: Crawler + ?Sized>(crawler: &C) -> Result<Vec<Notice>, CrawlError> {
    let crawled_notices = crawl(crawler).await?;

    Ok(crawled_notices
        .iter()
        .map(|notice| {
            create_notice(NewNotice {
                organization_name: &notice.organization_name,
                source_key: &notice.source_key,
                source_url: &notice.source_url,
                title: &notice.title,
                author: notice.author.as_deref(),
                category: notice.category.as_deref(),
                body_text: notice.body_text.as_deref(),
                is_pinned: notice.is_pinned,
                published_on: notice.published_on,
            })
        })
        .collect())
}
