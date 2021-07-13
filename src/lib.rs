#[macro_use]
extern crate diesel;

extern crate dotenv;

extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_diesel;


#[macro_use] 
extern crate serde_derive; 

pub mod schema;
pub mod models;


use dotenv::dotenv;
use diesel::prelude::*;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;


pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok(); 


    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Failed to create pool.")
}

pub struct DbConn(PooledConnection<ConnectionManager<PgConnection>>);


impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = (); 

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {

        let pool = request.guard::<State<Pool<ConnectionManager<PgConnection>>>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
