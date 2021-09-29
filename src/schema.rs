table! {
    users (id) {
        id -> Nullable<Integer>,
        name -> Text,
        last_name -> Nullable<Text>,
        id_doc_type -> Integer,
        document -> Text,
        residence -> Integer,
        birthday -> Date,
        email -> Text,
        phone -> Text,
        username -> Text,
        passhash -> Text,
    }
}
