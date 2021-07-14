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
pub mod form;


use dotenv::dotenv;
use diesel::prelude::*;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::{Status, RawStr};
use rocket::request::{self, FromRequest,FromFormValue};
use std::env;
use std::ops::Deref;

use chrono::NaiveDate;
use chrono::NaiveTime;
use chrono::NaiveDateTime;


pub fn create_db_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok(); 


    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::new(manager).expect("Failed to create pool.")
}


//Following these notes to implement traits on types I don't own
// https://stackoverflow.com/questions/25413201/how-do-i-implement-a-trait-i-dont-own-for-a-type-i-dont-own
// https://github.com/SergioBenitez/Rocket/issues/602#issuecomment-380497269

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

pub struct NaiveDateForm(NaiveDate);
pub struct NaiveTimeForm(NaiveTime);
pub struct NaiveDateTimeForm(NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(date) = NaiveDate::parse_from_str(&decoded, "%Y-%m-%d") {
            return Ok(NaiveDateForm(date));
        }
        Err(form_value)
    }
}

impl<'v> FromFormValue<'v> for NaiveTimeForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M:%S%.3f") {
            // if time.nanosecond() >= 1_000_000_000 {
            //     return Err(form_value);
            // }
            return Ok(NaiveTimeForm(time));
        }
        if let Ok(time) = NaiveTime::parse_from_str(&decoded, "%H:%M") {
            return Ok(NaiveTimeForm(time));
        }
        Err(form_value)
    }
}

impl<'v> FromFormValue<'v> for NaiveDateTimeForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTimeForm, &'v RawStr> {
        let decoded = form_value.url_decode().map_err(|_| form_value)?;
        if decoded.len() < "0000-00-00T00:00".len() {
            return Err(form_value)
        }
        let date = NaiveDateForm::from_form_value(RawStr::from_str(&decoded[.."0000-00-00".len()]))
            .map_err(|_| form_value)?;
        let time = NaiveTimeForm::from_form_value(RawStr::from_str(&decoded["0000-00-00T".len()..]))
            .map_err(|_| form_value)?;
        Ok(NaiveDateTimeForm(NaiveDateTime::new(*date, *time)))
    }
}

impl Deref for NaiveDateForm {
    type Target = NaiveDate;
    fn deref(&self) -> &NaiveDate {
        &self.0
    }
}

impl Deref for NaiveTimeForm {
    type Target = NaiveTime;
    fn deref(&self) -> &NaiveTime {
        &self.0
    }
}

impl Deref for NaiveDateTimeForm {
    type Target = NaiveDateTime;
    fn deref(&self) -> &NaiveDateTime {
        &self.0
    }
}
