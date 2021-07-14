
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;

extern crate diesel; 
extern crate scubalib;
extern crate rocket_contrib;
extern crate tera;
extern crate chrono;

use diesel::prelude::*;
use scubalib::*;
use scubalib::models::*;

use rocket_contrib::templates::Template;
use rocket::response::NamedFile;
use rocket::request::Form;
use rocket::response::Redirect;

use tera::Context;


#[derive(FromForm)]
struct NewDiveForm {
    depth: f64,
    startingo2: f64,
    endingo2: f64,
    #[form(field = "textarea0")]
    location: String,
    #[form(field = "textarea1")]
    description: String,
    divedate: NaiveDateForm,
    divetime: NaiveTimeForm,
    temperature: f64
}

#[derive(FromForm)]
struct NewFilterForm {
    #[form(field = "textarea1")]
    filtertext: String,
}


fn main() {
  rocket::ignite().manage(create_db_pool())
    .mount("/", routes![index, newdiveentry, diveroute, getfile, filterroute])
    .attach(Template::fairing())
    .launch();
}


#[get("/")]
fn index(connection: DbConn) -> Template {
    use schema::dive::dsl::*;
    
    let mut context = Context::new();
    
    let dive_list = dive.load::<Dive>(&*connection).expect("Error Loading Dives");

    context.insert("dives", &dive_list);
    
    Template::render("table", context.into_json())
}

#[get("/newdive")]
fn newdiveentry() -> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}

#[get("/sorttable.js")]
fn getfile() -> Option<NamedFile> {
    NamedFile::open("static/sorttable.js").ok()
}

#[post("/filter", data = "<filterform>")]
fn filterroute(filterform: Form<NewFilterForm>, connection:DbConn) -> Template 
{
    let mut context = Context::new();

    use schema::dive::dsl::*;
    let dive_list = dive.load::<Dive>(&*connection).expect("Error Loading Dives");
    let filtered_dives = dive_list.iter().filter(|x| x.divedescription.contains(&filterform.filtertext));
    
    let filetered_dive_vector : Vec<&Dive> = filtered_dives.collect();
    context.insert("dives", &filetered_dive_vector);

    Template::render("table", context.into_json())
}

#[post("/", data = "<diveform>")]
fn diveroute(diveform: Form<NewDiveForm>, connection: DbConn) -> Redirect {
    
    let new_dive = NewDive 
    {
        depth : diveform.depth,
        startingo2 : diveform.startingo2,
        endingo2 : diveform.endingo2,
        divelocation : diveform.location.clone(),
        divedatetime : chrono::NaiveDateTime::new(*diveform.divedate,*diveform.divetime),
        temperature : diveform.temperature,
        divedescription : diveform.description.clone()
    };
    
    use schema::dive::dsl::*;
    diesel::insert_into(dive).values(&new_dive).execute(&*connection).expect("Error inserting sample dive");
    
    Redirect::to("/")
}
