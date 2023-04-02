// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hashed_pass -> Nullable<Text>,
    }
}
