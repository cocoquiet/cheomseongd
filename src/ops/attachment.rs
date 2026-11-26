use crate::db::establish_connection;
use crate::models::attachment::{Attachment, NewAttachment};
use crate::schema::attachments;
use diesel::prelude::*;

pub fn create_attachment(new_attachment: NewAttachment) -> Attachment {
    let connection = &mut establish_connection();
    diesel::insert_into(attachments::table)
        .values(&new_attachment)
        .get_result(connection)
        .expect("Error saving new attachment")
}

pub fn get_attachment_by_id(attachment_id: i64) -> QueryResult<Attachment> {
    let connection = &mut establish_connection();
    attachments::table
        .filter(attachments::id.eq(attachment_id))
        .select(Attachment::as_select())
        .first::<Attachment>(connection)
}

pub fn get_attachments_by_notice_id(notice_id: i64) -> QueryResult<Vec<Attachment>> {
    let connection = &mut establish_connection();
    attachments::table
        .filter(attachments::notice_id.eq(notice_id))
        .select(Attachment::as_select())
        .load::<Attachment>(connection)
}

pub fn delete_attachment(attachment_id: i64) -> QueryResult<usize> {
    let connection = &mut establish_connection();
    diesel::delete(attachments::table.filter(attachments::id.eq(attachment_id))).execute(connection)
}
