// @generated automatically by Diesel CLI.

diesel::table! {
    tweets (id) {
        id -> Int4,
        #[max_length = 255]
        message -> Varchar,
        created_at -> Timestamp,
    }
}
