use super::{CrawlError, CrawledNotice, Crawler};
use crate::models::{
    attachment::NewAttachment,
    image::NewImage,
    notice::{NewNotice, Notice, UpdateNotice},
};
use crate::ops::{
    attachment::{create_attachment, delete_attachments_by_notice_id},
    image::{create_image, delete_images_by_notice_id},
    notice::{create_notice, get_notice_by_source, update_notice},
};

/// Runs one crawler and returns its normalized notices.
///
/// Keeping persistence outside this function lets the runner report source
/// failures without coupling crawler implementations to Diesel models.
pub async fn crawl<C: Crawler + ?Sized>(crawler: &C) -> Result<Vec<CrawledNotice>, CrawlError> {
    crawler.crawl().await
}

/// Crawls one source and inserts notices plus their attachments and images
/// through the existing DB operations.
///
/// Database errors follow the current `create_*` operation behavior, which panics on failure.
pub async fn run<C: Crawler + ?Sized>(crawler: &C) -> Result<Vec<Notice>, CrawlError> {
    let crawled_notices = crawl(crawler).await?;
    let mut saved_notices = Vec::with_capacity(crawled_notices.len());

    for crawled_notice in &crawled_notices {
        let notice = match get_notice_by_source(
            &crawled_notice.organization_name,
            &crawled_notice.source_key,
        )
        .expect("Error looking up notice by source")
        {
            Some(existing) => update_notice(
                existing.id,
                UpdateNotice {
                    organization_name: Some(&crawled_notice.organization_name),
                    source_key: Some(&crawled_notice.source_key),
                    source_url: Some(&crawled_notice.source_url),
                    title: Some(&crawled_notice.title),
                    author: crawled_notice.author.as_deref(),
                    category: crawled_notice.category.as_deref(),
                    body_text: crawled_notice.body_text.as_deref(),
                    is_pinned: Some(crawled_notice.is_pinned),
                    published_on: crawled_notice.published_on,
                },
            )
            .expect("Error updating notice"),
            None => create_notice(NewNotice {
                organization_name: &crawled_notice.organization_name,
                source_key: &crawled_notice.source_key,
                source_url: &crawled_notice.source_url,
                title: &crawled_notice.title,
                author: crawled_notice.author.as_deref(),
                category: crawled_notice.category.as_deref(),
                body_text: crawled_notice.body_text.as_deref(),
                is_pinned: crawled_notice.is_pinned,
                published_on: crawled_notice.published_on,
            }),
        };

        delete_attachments_by_notice_id(notice.id).expect("Error replacing notice attachments");
        delete_images_by_notice_id(notice.id).expect("Error replacing notice images");

        for attachment in &crawled_notice.attachments {
            create_attachment(NewAttachment {
                notice_id: notice.id,
                source_key: &attachment.source_key,
                file_name: &attachment.file_name,
                download_url: &attachment.download_url,
                display_order: attachment.display_order,
                content_type: attachment.content_type.as_deref(),
            });
        }

        for image in &crawled_notice.images {
            create_image(NewImage {
                notice_id: notice.id,
                source_url: &image.source_url,
                alt_text: image.alt_text.as_deref(),
                display_order: image.display_order,
            });
        }

        saved_notices.push(notice);
    }

    Ok(saved_notices)
}
