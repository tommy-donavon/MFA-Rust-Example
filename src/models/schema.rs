table! {
    user (email) {
        email -> Text,
        password -> Text,
    }
}

table! {
    user_code (id) {
        id -> Integer,
        code -> Text,
        user_email -> Text,
    }
}

joinable!(user_code -> user (user_email));

allow_tables_to_appear_in_same_query!(
    user,
    user_code,
);
