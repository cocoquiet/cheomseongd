use cheomseongd::models::attachment::NewAttachment;
use cheomseongd::models::image::NewImage;
use cheomseongd::models::notice::{NewNotice, UpdateNotice};
use cheomseongd::ops::{attachment, image, notice};
use diesel::result::Error;
use std::time::{SystemTime, UNIX_EPOCH};

struct NoticeCleanup(i64);

impl Drop for NoticeCleanup {
    fn drop(&mut self) {
        let _ = notice::delete_notice(self.0);
    }
}

#[test]
fn crud_flow_for_database_operations() {
    let unique_key = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX epoch")
        .as_nanos()
        .to_string();
    let source_key = format!("integration-test-{unique_key}");

    let created_notice = notice::create_notice(NewNotice {
        organization_name: "Integration test organization",
        source_key: &source_key,
        source_url: "https://example.test/notices/integration-test",
        title: "Created notice",
        author: Some("Test author"),
        category: Some("Test category"),
        body_text: Some("Test body"),
        is_pinned: false,
        published_on: None,
    });
    let _cleanup = NoticeCleanup(created_notice.id);

    assert_eq!(created_notice.title, "Created notice");
    let fetched_notice = notice::get_notice_by_id(created_notice.id).unwrap();
    assert_eq!(fetched_notice.source_key, source_key);
    assert!(
        notice::get_notices()
            .unwrap()
            .iter()
            .any(|item| item.id == created_notice.id)
    );

    let updated_notice = notice::update_notice(
        created_notice.id,
        UpdateNotice {
            organization_name: None,
            source_key: None,
            source_url: None,
            title: Some("Updated notice"),
            author: None,
            category: None,
            body_text: None,
            is_pinned: Some(true),
            published_on: None,
        },
    )
    .unwrap();
    assert_eq!(updated_notice.title, "Updated notice");
    assert!(updated_notice.is_pinned);
    assert_eq!(updated_notice.author.as_deref(), Some("Test author"));

    let attachment_source_key = format!("attachment-{unique_key}");
    let created_attachment = attachment::create_attachment(NewAttachment {
        notice_id: created_notice.id,
        source_key: &attachment_source_key,
        file_name: "test-document.pdf",
        download_url: "https://example.test/files/test-document.pdf",
        display_order: 0,
        content_type: Some("application/pdf"),
    });
    assert_eq!(
        attachment::get_attachment_by_id(created_attachment.id)
            .unwrap()
            .file_name,
        "test-document.pdf"
    );
    assert!(
        attachment::get_attachments_by_notice_id(created_notice.id)
            .unwrap()
            .iter()
            .any(|item| item.id == created_attachment.id)
    );
    assert_eq!(
        attachment::delete_attachment(created_attachment.id).unwrap(),
        1
    );
    assert!(matches!(
        attachment::get_attachment_by_id(created_attachment.id),
        Err(Error::NotFound)
    ));

    let image_source_url = format!("https://example.test/images/{unique_key}.png");
    let created_image = image::create_image(NewImage {
        notice_id: created_notice.id,
        source_url: &image_source_url,
        alt_text: Some("Integration test image"),
        display_order: 0,
    });
    assert_eq!(
        image::get_image_by_id(created_image.id)
            .unwrap()
            .alt_text
            .as_deref(),
        Some("Integration test image")
    );
    assert!(
        image::get_images_by_notice_id(created_notice.id)
            .unwrap()
            .iter()
            .any(|item| item.id == created_image.id)
    );
    assert_eq!(image::delete_image(created_image.id).unwrap(), 1);
    assert!(matches!(
        image::get_image_by_id(created_image.id),
        Err(Error::NotFound)
    ));

    assert_eq!(notice::delete_notice(created_notice.id).unwrap(), 1);
    assert!(matches!(
        notice::get_notice_by_id(created_notice.id),
        Err(Error::NotFound)
    ));
}
