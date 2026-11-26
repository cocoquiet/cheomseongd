use crate::db::establish_connection;
use crate::models::image::{Image, NewImage};
use crate::schema::images;
use diesel::prelude::*;

pub fn create_image(new_image: NewImage) -> Image {
    let connection = &mut establish_connection();
    diesel::insert_into(images::table)
        .values(&new_image)
        .get_result(connection)
        .expect("Error saving new image")
}

pub fn get_image_by_id(image_id: i64) -> QueryResult<Image> {
    let connection = &mut establish_connection();
    images::table
        .filter(images::id.eq(image_id))
        .select(Image::as_select())
        .first::<Image>(connection)
}

pub fn get_images_by_notice_id(notice_id: i64) -> QueryResult<Vec<Image>> {
    let connection = &mut establish_connection();
    images::table
        .filter(images::notice_id.eq(notice_id))
        .select(Image::as_select())
        .load::<Image>(connection)
}

pub fn delete_image(image_id: i64) -> QueryResult<usize> {
    let connection = &mut establish_connection();
    diesel::delete(images::table.filter(images::id.eq(image_id))).execute(connection)
}
