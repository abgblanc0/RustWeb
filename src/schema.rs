// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id, user_id) {
        id -> Int4,
        user_id -> Int4,
        ctime -> Nullable<Timestamptz>,
        content -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        ctime -> Nullable<Timestamptz>,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::joinable!(comment -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    users,
);
