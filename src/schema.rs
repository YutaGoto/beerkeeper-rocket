table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        location -> Nullable<Varchar>,
        max_size -> Int4,
        budget -> Nullable<Varchar>,
        description -> Nullable<Text>,
        start_at -> Nullable<Timestamp>,
        end_at -> Nullable<Timestamp>,
        organizer_id -> Int4,
    }
}

table! {
    participations (id) {
        id -> Int4,
        event_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Nullable<Varchar>,
    }
}

joinable!(events -> users (organizer_id));
joinable!(participations -> events (event_id));
joinable!(participations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    participations,
    users,
);
