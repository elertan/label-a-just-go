table! {
    event (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    event_invitation (id) {
        id -> Int4,
        person_id -> Int4,
        event_id -> Int4,
        token -> Nullable<Uuid>,
        status -> Int4,
    }
}

table! {
    person (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    photo (id) {
        id -> Int4,
        person_id -> Int4,
        filename -> Varchar,
    }
}

joinable!(event_invitation -> event (event_id));
joinable!(event_invitation -> person (person_id));
joinable!(photo -> person (person_id));

allow_tables_to_appear_in_same_query!(
    event,
    event_invitation,
    person,
    photo,
);
