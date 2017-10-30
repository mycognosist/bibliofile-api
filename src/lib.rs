// 'src/lib.rs'

// Import Diesel lib for database interaction &
// code generation for the Diesel API
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

// Access to ENV vars
extern crate dotenv;

// Connection Pool lib & plugin for Diesel
extern crate r2d2;
extern crate r2d2_diesel;

// Rocket lib & API
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// These mod declarations re-export those files / modules
// Schema contains the 'infer_schema!' macro to generate table APIs
// Models are the database models we setup in 'src/models.rs'
// We re-export so any file that includes 'lib.rs' may have access as well
pub mod schema;
pub mod models;

// Bring each necessary module into scope
// These will be modules we reference in the rest of the file
use dotenv::dotenv;
use r2d2::{Config, Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
// Added by mycognosist
use diesel::sqlite::SqliteConnection;
//
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

// Set up a connection pool
pub fn create_db_pool() -> Pool<ConnectionManager<SqliteConnection>> {
    dotenv().ok(); // Fetch ENV variables

    // Pull DATABASE_URL env var
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Set default config for connection pool
    // r2d2::Config Docs: https://docs.rs/r2d2/0.7.4/r2d2/struct.Config.html
    let config = Config::default();

    // Create a connection pool manager for a Sqlite connection
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    // Create the pool with the default config & the r2d2_diesel connection manager
    Pool::new(config, manager).expect("Failed to create pool.")
}

// This is the struct we will be passing around as a request guard
pub struct DbConn(PooledConnection<ConnectionManager<SqliteConnection>>);

// This allows our connection pool to become a request guard
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    // This is the required method that does all the dirty work
    // Because FromRequest is a "validation", we can put any logic here
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<SqliteConnection>>>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
