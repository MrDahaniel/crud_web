CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255),
    id_doc_type INT NOT NULL,
    document VARCHAR(255) NOT NULL,
    residence INT NOT NULL,
    birthday DATE NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255) NOT NULL,
    username VARCHAR(30) NOT NULL,
    passhash VARCHAR(255) NOT NULL
)
