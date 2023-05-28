#[macro_use] extern crate rocket;
use diesel::RunQueryDsl;
use rocket_sync_db_pools::database;

use rocket::{serde::json::Json};
//models
mod models;
use models::{User, NewUser};
//schema
mod schema;
use schema::client;
#[database("my_db")]
pub struct Db(diesel::PgConnection);
#[post("/sign-up",format = "json", data="<user>")]
async fn create_user(user:Json<User>, conn:Db) -> Json<NewUser>{
    let user = user.into_inner();
    let  new_user = NewUser{
        client_id: user.client_id,
        first_name: user.first_name.to_string() ,
        last_name: user.last_name.to_string(),
        client_password:user.client_password.to_string(),
        email: user.email.to_string(),
        admin_privilege: false,
    };
    conn.run(move |c| {
        diesel::insert_into(client::table).values(new_user).get_result::<NewUser>(c)
    }).await.map(Json).expect("the information was not valid")
    }















#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .mount("/", routes![])
    
}
