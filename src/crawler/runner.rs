use super::{CrawlError, CrawledNotice, Crawler};
use crate::db::establish_connection;
use crate::models::{
    attachment::NewAttachment,
    image::NewImage,
    notice::{NewNotice, Notice, UpdateNotice},
};
use crate::schema::{attachments, images, notices};
use diesel::prelude::*;
use diesel::result::OptionalExtension;

/// Runs one crawler and returns its normalized notices.
///
/// Keeping persistence outside this function lets the runner report source
/// failures without coupling crawler implementations to Diesel models.
pub async fn crawl<C: Crawler + ?Sized>(crawler: &C) -> Result<Vec<CrawledNotice>, CrawlError> {
    crawler.crawl().await
}

/// Crawls one source and transactionally inserts or updates notices and related media.
pub async fn run<C: Crawler + ?Sized>(crawler: &C) -> Result<Vec<Notice>, CrawlError> {
    let crawled_notices = crawl(crawler).await?;
    let mut saved_notices = Vec::with_capacity(crawled_notices.len());
    let connection = &mut establish_connection();

    for crawled_notice in &crawled_notices {
        let notice = connection
            .transaction::<Notice, diesel::result::Error, _>(|connection| {
                let existing = notices::table
                    .filter(notices::organization_name.eq(&crawled_notice.organization_name))
                    .filter(notices::source_key.eq(&crawled_notice.source_key))
                    .select(Notice::as_select())
                    .first::<Notice>(connection)
                    .optional()?;

                let notice = match existing {
                    Some(existing) => diesel::update(notices::table.find(existing.id))
                        .set(UpdateNotice {
                            organization_name: Some(&crawled_notice.organization_name),
                            source_key: Some(&crawled_notice.source_key),
                            source_url: Some(&crawled_notice.source_url),
                            title: Some(&crawled_notice.title),
                            author: crawled_notice.author.as_deref(),
                            category: crawled_notice.category.as_deref(),
                            body_text: crawled_notice.body_text.as_deref(),
                            is_pinned: Some(crawled_notice.is_pinned),
                            published_on: crawled_notice.published_on,
                        })
                        .get_result::<Notice>(connection)?,
                    None => diesel::insert_into(notices::table)
                        .values(NewNotice {
                            organization_name: &crawled_notice.organization_name,
                            source_key: &crawled_notice.source_key,
                            source_url: &crawled_notice.source_url,
                            title: &crawled_notice.title,
                            author: crawled_notice.author.as_deref(),
                            category: crawled_notice.category.as_deref(),
                            body_text: crawled_notice.body_text.as_deref(),
                            is_pinned: crawled_notice.is_pinned,
                            published_on: crawled_notice.published_on,
                        })
                        .get_result::<Notice>(connection)?,
                };

                diesel::delete(attachments::table.filter(attachments::notice_id.eq(notice.id)))
                    .execute(connection)?;
                diesel::delete(images::table.filter(images::notice_id.eq(notice.id)))
                    .execute(connection)?;

                for attachment in &crawled_notice.attachments {
                    diesel::insert_into(attachments::table)
                        .values(NewAttachment {
                            notice_id: notice.id,
                            source_key: &attachment.source_key,
                            file_name: &attachment.file_name,
                            download_url: &attachment.download_url,
                            display_order: attachment.display_order,
                            content_type: attachment.content_type.as_deref(),
                        })
                        .execute(connection)?;
                }

                for image in &crawled_notice.images {
                    diesel::insert_into(images::table)
                        .values(NewImage {
                            notice_id: notice.id,
                            source_url: &image.source_url,
                            alt_text: image.alt_text.as_deref(),
                            display_order: image.display_order,
                        })
                        .execute(connection)?;
                }

                Ok(notice)
            })
            .map_err(|error| CrawlError::new(crawler.source_id(), error.to_string()))?;
        saved_notices.push(notice);
    }

    Ok(saved_notices)
}
