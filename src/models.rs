use super::schema::dive;
use chrono::NaiveDateTime;

#[derive(Debug, Queryable, Serialize)]
pub struct Dive {
    pub id: i32,
    pub depth: f64,
    pub startingo2: f64,
    pub endingo2: f64,
    pub divelocation: String,
    pub divedatetime : NaiveDateTime,
    pub temperature: f64,
    pub divedescription: String
}

#[derive(Debug, Insertable)]
#[table_name="dive"]
pub struct NewDive {
    pub depth: f64,
    pub startingo2: f64,
    pub endingo2: f64,
    pub divelocation: String,
    pub divedatetime : NaiveDateTime,
    pub temperature: f64,
    pub divedescription: String
}
