#[macro_use] extern crate rocket;
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl};
use rocket_sync_db_pools::database;

use rocket::{serde::json::Json, fs::FileServer, fs::relative};
use rocket::http::{Cookie, CookieJar};
//auth
mod authentication;
//models
mod models;
use models::user::{User};
// use models::client_to_room::{ClientToRoom,NewClientToRoom};
//schema
mod schema;
use schema::client::{self};
#[database("my_db")]
// create user will be expanded for case in which the user can not be login because they do not have uniq data 
pub struct Db(diesel::PgConnection);

#[post("/admin")]
async fn activate_admin(conn:Db, jar: &CookieJar<'_>) -> Json<bool>{
    let hold:i32 = jar.get_private("user_id").map( |cookie|  cookie.value().parse().ok()).unwrap().unwrap();
    let user_result:Result<User, Error> = conn.run(move |c| {
        client::table::find(client::table, hold).first::<User>(c)
    }).await;
    match user_result{
        Ok(item) => {
            if item.admin_privilege == true {
                create_cookie(jar, "admin_access".to_string(),  "True".to_string());
            }         
            Json(true)},
        Err(_) => Json(false),
    }

}
//setup defualt feature for the websiteew
pub fn create_cookie(jar: &CookieJar<'_>, key: String  , id : String){
    let cookie = Cookie::build(key, id.to_string()).http_only(true).secure(true);
    jar.add_private(cookie.finish());
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .mount("/admin", routes![activate_admin])
    .mount("/", authentication::routes())
    .mount("/", FileServer::from(relative!("static")))
}
//
//|user| Json(user)