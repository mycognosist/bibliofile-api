# Bibliofile: A RESTful API written in Rocket for book collectors

I'm using this project to learn more about [Rocket](https://rocket.rs), a Rust web framework written by [Sergio Benitez](https://sergio.bz/), and [Diesel](https://diesel.rs), an ORM & Query Builder by Sean Griffin.

The [Rust blog tutorial series](https://notryanb.github.io/rust-blog-series-2.html) forms the basis of the code for this project, although I have adapted it to work with a Sqlite database (as opposed to Postgres).

The goal is to implement a RESTful API for cataloging one's book collection. The API will operate on CRUD (Create, Read, Update, Delete) principles.

| HTTP Method | URI                                     | Action                  |
|-------------|-----------------------------------------|-------------------------|
| GET         | http://[hostname]/api/v1.0/books        | Retrieve list of books  |
| GET         | http://[hostname]/api/v1.0/books/[isbn] | Retrieve a book         |
| POST        | http://[hostname]/api/v1.0/books        | Create a book           |
| PUT         | http://[hostname]/api/v1.0/books/[isbn] | Update an existing book |
| DELETE      | http://[hostname]/api/v1.0/books/[isbn] | Delete a book           |
