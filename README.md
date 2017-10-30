# Bibliofile: A RESTful API written in Rocket for book collectors

I'm using this project to learn more about [Rocket](https://rocket.rs), a Rust web framework written by [Sergio Benitez](https://sergio.bz/), and [Diesel](https://diesel.rs), an ORM & Query Builder by Sean Griffin.

The [Rust blog tutorial series](https://notryanb.github.io/rust-blog-series-2.html) by [Ryan Blecher](https://twitter.com/Options_R) forms the basis of the code for this project, although I have adapted it to work with a Sqlite database (as opposed to Postgres).

The code is currently up to date with Part II of the tutorial series.

The goal is to implement a RESTful API for cataloging one's book collection. The API will operate on CRUD (Create, Read, Update, Delete) principles.

| HTTP Method | URI                                     | Action                  |
|-------------|-----------------------------------------|-------------------------|
| GET         | http://[hostname]/api/v1.0/books        | Retrieve list of books  |
| GET         | http://[hostname]/api/v1.0/books/[isbn] | Retrieve a book         |
| POST        | http://[hostname]/api/v1.0/books        | Create a book           |
| PUT         | http://[hostname]/api/v1.0/books/[isbn] | Update an existing book |
| DELETE      | http://[hostname]/api/v1.0/books/[isbn] | Delete a book           |

## Setup Instructions

1. Clone the repo and change into it

git clone https://github.com/mycognosist/bibliofile-api.git && cd bibliofile-api

2. Set environment variable (may not be necessary)

source $HOME/.cargo/env

3. Install diesel_cli

cargo install diesel_cli --no-default-features --features sqlite

4. Create .env and add db url

vim .env
DATABASE_URL=/path/to/your/database.db

5. Setup diesel (creates migrations folder)

diesel setup

6. Create migrations

diesel migration generate create_users_and_books

7. Define db schemas

vim migrations/...create_users_and_books/up.sql
vim migrations/...create_users_and_books/down.sql

8. Run migrations

diesel migration run

9. Seed the database (optional)

cargo run --bin seed
