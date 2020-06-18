table! {
    auth (user_id) {
        user_id -> Int4,
        enc_password -> Varchar,
    }
}

table! {
    chats (chat_id) {
        chat_id -> Int8,
        messages -> Jsonb,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        settings -> Jsonb,
        visible_name -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(
    auth,
    chats,
    users,
);
