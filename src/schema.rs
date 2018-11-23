table! {
    base_actors (id) {
        id -> Int4,
        display_name -> Varchar,
        profile_url -> Varchar,
        inbox_url -> Varchar,
        outbox_url -> Varchar,
        local_user -> Nullable<Int4>,
        follow_policy -> Varchar,
        original_json -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    base_posts (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        media_type -> Varchar,
        posted_by -> Int4,
        icon -> Nullable<Int4>,
        visibility -> Varchar,
        original_json -> Jsonb,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    files (id) {
        id -> Int4,
        file_path -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    followers (id) {
        id -> Int4,
        follower -> Int4,
        follows -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    images (id) {
        id -> Int4,
        width -> Int4,
        height -> Int4,
        file_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    links (id) {
        id -> Int4,
        href -> Varchar,
        href_lang -> Varchar,
        height -> Nullable<Int4>,
        width -> Nullable<Int4>,
        preview -> Nullable<Text>,
        base_post -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    personas (id) {
        id -> Int4,
        default_visibility -> Varchar,
        is_searchable -> Bool,
        avatar -> Nullable<Int4>,
        shortname -> Varchar,
        base_actor -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        date -> Text,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(base_actors -> users (local_user));
joinable!(base_posts -> base_actors (posted_by));
joinable!(base_posts -> images (icon));
joinable!(images -> files (file_id));
joinable!(links -> base_posts (base_post));
joinable!(personas -> base_actors (base_actor));
joinable!(personas -> images (avatar));

allow_tables_to_appear_in_same_query!(
    base_actors,
    base_posts,
    files,
    followers,
    images,
    links,
    personas,
    posts,
    users,
);
