// @generated automatically by Diesel CLI.

diesel::table! {
    service_contexts (id) {
        id -> Int4,
        maintenance -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(service_contexts, users,);
