table! {
    users (id) {
        id -> Varchar,
        user_name -> Varchar,
        email -> Varchar,
        password -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
