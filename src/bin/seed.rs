
extern crate dblib;

extern crate diesel;

use diesel::prelude::*;
use dblib::*;
use dblib::models::*;
use std::time::SystemTime;


fn main() {

    use schema::dive::dsl::*;

    let connection = create_db_pool().get().unwrap();

    diesel::delete(dive).execute(&*connection).expect("Error deleting posts");

     // Create personal login
     let sample_dive = NewDive {
        depth: 50.0,
        startingo2: 5.0,
        endingo2: 1.0,
        divelocation: "Jamaica".to_string(),
        divedatetime : SystemTime::now(),
        temperature: 58.0,
        divedescription: "Took Awhile".to_string()
    };


    diesel::insert_into(dive).values(&sample_dive).execute(&*connection).expect("Error inserting sample dive");
}
  