diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        created_by -> Nullable<Int4>,
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    posts_tags (post_id, tag) {
        post_id -> Int4,
        tag -> Varchar,
    }
}

diesel::joinable!(posts -> users (created_by));
diesel::joinable!(posts_tags -> posts (post_id));
diesel::allow_tables_to_appear_in_same_query!(users, posts, posts_tags);