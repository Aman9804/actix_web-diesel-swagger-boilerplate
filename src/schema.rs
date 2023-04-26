// @generated automatically by Diesel CLI.


diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        sex -> Text,
        mobile -> Text,
        created_at -> Timestamp,
    }
}



