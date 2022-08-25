CREATE TABLE user (
    email NVARCHAR(100) Primary Key NOT NULL,
    password NVARCHAR(500) NOT NULL
);

CREATE TABLE user_code (
    id INTEGER PRIMARY KEY NOT NULL,
    code NVARCHAR(5) UNIQUE NOT NULL,
    user_email NVARCHAR(100) NOT NULL,
    FOREIGN KEY (user_email) REFERENCES user (email)
);
