// @generated automatically by Diesel CLI.

diesel::table! {
    config (id) {
        id -> Integer,
        token -> Text,
        dev_channel -> Text,
    }
}
