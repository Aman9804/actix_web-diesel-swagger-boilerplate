// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        sex -> Text,
        created_at -> Timestamp,
    }
}
