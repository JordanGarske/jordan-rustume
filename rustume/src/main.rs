#[macro_use] extern crate rocket;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use rocket_sync_db_pools::database;

use rocket::{serde::json::Json, fs::FileServer, fs::relative};
//models
mod models;

use models::user::{User, NewUser};
//schema
mod schema;
use schema::client::{self, first_name};
#[database("my_db")]
pub struct Db(diesel::PgConnection);
#[post("/sign-up",format = "json", data="<user>")]
async fn create_user(user:Json<NewUser>, conn:Db) -> Json<User>{
    conn.run(move |c| {
        diesel::insert_into(client::table).values(user.into_inner()).get_result::<User>(c)
    }).await.map(Json).expect("the information was not valid")
    }
#[get("/login")]
async fn login_user( conn:Db) -> Json<User>{
    // client::table::filter(first_name.eq("Sean")).load;
    // /client::table::load::<NewUser>(client::table,c)
    conn.run(move |c| {
        client::table::filter(client::table,first_name.eq("other")).first::<User>(c)
    }).await.map(Json).unwrap_or(Json(User{  client_id:1,  first_name: "f".to_string(),  last_name: "f".to_string(),  client_password: "f".to_string(),  email: "f".to_string(),  admin_privilege:false}))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .mount("/", routes![create_user,login_user])
    .mount("/", FileServer::from(relative!("static")))
}
//
//|user| Json(user)