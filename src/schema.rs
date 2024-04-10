// @generated automatically by Diesel CLI.

diesel::table! {
    confirmations (id) {
        id -> Uuid,
        #[max_length = 50]
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    userrs (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Text,
        hashed_password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    confirmations,
    userrs,
    users,
);
