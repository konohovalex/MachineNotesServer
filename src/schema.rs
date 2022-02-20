table! {
    user_account (user_id) {
        user_id -> Text,
        user_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        password_hash -> Nullable<Text>,
        password_hash_salt -> Nullable<Text>,
        password_hash_algorithm -> Nullable<Text>,
        access_token -> Text,
        refresh_token -> Text,
    }
}
