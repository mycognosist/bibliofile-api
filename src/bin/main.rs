// 'src/bin/main.rs'


#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate bibliolib;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use diesel::prelude::*;
use bibliolib::*;
use bibliolib::models::*;
// Bring Template and Context into scope
use rocket_contrib::Template;
use tera::Context;

fn main() {
    rocket::ignite()
        .manage(create_db_pool()) // Register connection pool with Managed State
        .mount("/", routes![index])
        .attach(Template::fairing()) // This lets us use templating
        .launch();
}

// DbConn Request Guard
// The route now has access to a database connection
// It's dereferrenced when passed into the 'load()' method
#[get("/")]
fn index(connection: DbConn) -> Template {
    use schema::books::dsl::*;
    use schema::users::dsl::*;
    
    let mut context = Context::new();

    // 'load()' returns all records from each table it is called on
    // The 'books::dsl::*' allows us to use 'books' instead of 'books::table'
    // The types <Book> and <User> are imported by 'bibliolib::models::*;
    let book_list = books
        .load::<Book>(&*connection)
        .expect("Error loading books");
    let user_list = users
        .load::<User>(&*connection)
        .expect("Error loading users");

    // Make book and user lists available to tera for templating
    context.add("books", &book_list);
    context.add("users", &user_list);
    
    Template::render("base", &context)
}
