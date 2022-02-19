table! {
    auth_token (token) {
        user_id -> Text,
        token -> Text,
    }
}

table! {
    user_account (user_id) {
        user_id -> Text,
        user_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        password_hash -> Nullable<Text>,
        password_hash_salt -> Nullable<Text>,
        password_hash_algorithm -> Nullable<Text>,
    }
}

joinable!(auth_token -> user_account (user_id));

allow_tables_to_appear_in_same_query!(
    auth_token,
    user_account,
);
