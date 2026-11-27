// @generated automatically by Diesel CLI.

diesel::table! {
    attachments (id) {
        id -> Int8,
        notice_id -> Int8,
        #[max_length = 255]
        source_key -> Varchar,
        file_name -> Text,
        download_url -> Text,
        display_order -> Int4,
        #[max_length = 255]
        content_type -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    crawler_configs (crawler_id) {
        #[max_length = 100]
        crawler_id -> Varchar,
        interval_seconds -> Int8,
        notice_count -> Int4,
    }
}

diesel::table! {
    images (id) {
        id -> Int8,
        notice_id -> Int8,
        source_url -> Text,
        alt_text -> Nullable<Text>,
        display_order -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    notices (id) {
        id -> Int8,
        #[max_length = 200]
        organization_name -> Varchar,
        #[max_length = 255]
        source_key -> Varchar,
        source_url -> Text,
        title -> Text,
        #[max_length = 200]
        author -> Nullable<Varchar>,
        #[max_length = 200]
        category -> Nullable<Varchar>,
        body_text -> Nullable<Text>,
        is_pinned -> Bool,
        published_on -> Nullable<Date>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(attachments -> notices (notice_id));
diesel::joinable!(images -> notices (notice_id));

diesel::allow_tables_to_appear_in_same_query!(attachments, crawler_configs, images, notices,);
