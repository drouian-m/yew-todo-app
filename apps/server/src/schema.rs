// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        title -> Text,
        completed -> Bool,
        created_at -> Timestamp,
    }
}
