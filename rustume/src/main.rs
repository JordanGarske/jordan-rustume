#[macro_use] extern crate rocket;
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl};
use rocket::http::hyper::request;
use rocket_sync_db_pools::database;

use rocket::{serde::json::Json, fs::FileServer, fs::relative};
use rocket::http::{Cookie, CookieJar, Status, Header};
//auth
mod authentication;
//models
mod models;
mod user_rooms;
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
/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}
use rocket::http::Method;
use rocket::{get, routes};
use rocket_cors::{AllowedHeaders, AllowedOrigins};
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .attach(CorsFairing)
    .mount("/admin", routes![activate_admin,all_options])
    .mount("/", authentication::routes())
    .mount("/rooms", user_rooms::routes())
    .mount("/", FileServer::from(relative!("static")))
}
use rocket::{ Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
struct CorsFairing;
#[rocket::async_trait]
impl Fairing for CorsFairing {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>){
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            " http://localhost:4200",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}