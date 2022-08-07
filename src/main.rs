#[macro_use]
extern crate rocket;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::mongodb::{self, Client};
use mongodb::bson::doc;

#[derive(Database)]
#[database("mongodb")]
struct Logs(mongodb::Client);

#[get("/<id>")]
async fn read(mut db: Connection<Logs>, id: i64) -> Option<String> {
    mongodb::query("\"_id\": {\"$oid\": \"62efe55512fee4efeb7a07c0\"}")
    .bind(id)
    .fetch_one(&mut *db).await
    .and_then(|r| Ok(Log(r.try_get(0)?)))
    .ok();

    Option::Some("Ciao".parse().unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Logs::init()).mount("/", routes![read])
}