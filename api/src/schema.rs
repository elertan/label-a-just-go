table! {
    event (id) {
        id -> Int4,
        inv_id -> Int4,
    }
}

table! {
    invitation (id) {
        user_uuid -> Uuid,
        event_id -> Int4,
        id -> Int4,
        uuid -> Uuid,
    }
}

table! {
    picture (id) {
        file_name -> Varchar,
        id -> Int4,
        uuid -> Uuid,
    }
}

table! {
    registration (id) {
        id -> Int4,
        event_id -> Int4,
        user_uuid -> Uuid,
    }
}

table! {
    user_account (uuid) {
        first_name -> Varchar,
        surname -> Varchar,
        uuid -> Uuid,
    }
}

joinable!(event -> invitation (inv_id));
joinable!(invitation -> user_account (uuid));
joinable!(picture -> user_account (uuid));
joinable!(registration -> event (event_id));
joinable!(registration -> user_account (user_uuid));

allow_tables_to_appear_in_same_query!(
    event,
    invitation,
    picture,
    registration,
    user_account,
);
