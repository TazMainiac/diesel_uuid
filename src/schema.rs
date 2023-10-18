// @generated automatically by Diesel CLI.


diesel::table! {
    uuid_users (user_uid) {
        user_uid -> Uuid,
        name -> Text,
    }
}

