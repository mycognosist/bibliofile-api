// 'src/models.rs'

// This 'models' file will be imported into 'lib'
// Bring the 'schema' file into scope
use schema::{books, users};

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Book {
    pub id: i32,
    pub user_id: i32,
    pub author: String,
    pub title: String,
    pub read: bool,
}

#[derive(Debug, Insertable)]
#[table_name="books"]
pub struct NewBook {
    pub user_id: i32,
    pub author: String,
    pub title: String,
}
