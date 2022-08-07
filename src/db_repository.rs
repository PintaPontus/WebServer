#[macro_use] extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::mongodb::{self, Row};

#[derive(Database)]
#[database("mongodb")]
struct Logs(mongodb::Client);

pub fn query(mut db: Connection<Logs>, id: i64, query: &str)