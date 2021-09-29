use chrono::NaiveDate;

struct User {
    id: i32,
    name: String,
    last_name: Option<String>,
    id_doc_type: i32,
    document: String,
    residence: i32,
    birthday: NaiveDate,
    email: String,
    phone: String,
    username: String,
    passhash: String,
}

impl User {
    pub fn new(
        id: i32,
        name: String,
        last_name: Option<String>,
        id_doc_type: i32,
        document: String,
        residence: i32,
        birthday: NaiveDate,
        email: String,
        phone: String,
        username: String,
        passhash: String,
    ) -> User {
        return User {
            id,
            name,
            last_name,
            id_doc_type,
            document,
            residence,
            birthday,
            email,
            phone,
            username,
            passhash,
        };
    }
}
