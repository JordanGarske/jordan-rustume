#[macro_use] extern crate rocket;
use rocket_sync_db_pools::database;
#[database("my_db")]
pub struct Db(diesel::PgConnection);
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .mount("/", routes![])
    
}
