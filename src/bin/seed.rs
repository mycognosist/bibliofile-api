// 'src/bin/seed_db.rs'

extern crate bibliolib;
extern crate bcrypt;
extern crate diesel;
#[macro_use] extern crate fake;

use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use bibliolib::*;
use bibliolib::models::{NewBook, User, NewUser};

fn main() {
    use schema::books::dsl::*;
    use schema::users::dsl::*;

    let connection = create_db_pool().get().unwrap();
    let plain_text_pw = "testing";
    let hashed_password = match hash (plain_text_pw, DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => panic!("Error hashing")
    };

    diesel::delete(books).execute(&*connection).expect("Error deleting books");
    diesel::delete(users).execute(&*connection).expect("Error deleting users");

    // Randomly generate user info
    fn generate_user_info(pw: &str) -> NewUser {
        NewUser {
            username: fake!(Name.name),
            email: fake!(Internet.free_email),
            password: pw.to_string(),
        }
    }

    // Randomly generate book info
    fn generate_book_info(uid: i32) -> NewBook {
        NewBook {
            user_id: uid,
            author: fake!(Name.name),
            title: fake!(Lorem.sentence(1, 4)),
        }
    }

    // Create personal login
    let me = NewUser {
        username: "mycognosist".to_string(),
        email: "gnomad@cryptolab.net".to_string(),
        password: hashed_password.to_string(),
    };

    // Use Diesel Insert API to create INSERT statement & execute with connection
    diesel::insert(&me)
        .into(users)
        .execute(&*connection)
        .expect("Error inserting users");

    // Create 10 randomly generated users stored in a vec
    let new_user_list: Vec<NewUser> = (0..10)
        .map( |_| generate_user_info(&hashed_password))
        .collect();

    // Print list of generated users
    println!("[ Displaying {} users ]", &new_user_list.len());
    for user in &new_user_list {
        println!("----------");
        println!("{}", user.username);
        println!("{}", user.email);
    }

    // Insert vec of users
    diesel::insert(&new_user_list)
        .into(users)
        .execute(&*connection)
        .expect("Error inserting users");

    // Get a vec with newly inserted users
    let returned_users = users
        .limit(10)
        .load::<User>(&*connection)
        .expect("Error loading users");
    
    // Generate books for each of the new users
    let new_book_list: Vec<NewBook> = returned_users
        .into_iter()
        .map(|user| generate_book_info(user.id))
        .collect();

    // Print list of generated books
    println!("[ Displaying {} books ]", &new_book_list.len());
    for book in &new_book_list {
        println!("----------");
        println!("{}", book.title);
        println!("{}", book.author);
        println!("{}", book.user_id);
    }

    // Insert the books
    diesel::insert(&new_book_list)
        .into(books)
        .execute(&*connection)
        .expect("Error inserting books");
}
