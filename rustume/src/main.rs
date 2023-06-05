#[macro_use] extern crate rocket;
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use rocket_sync_db_pools::database;

use rocket::{serde::json::Json, fs::FileServer, fs::relative};
use rocket::http::{Cookie, CookieJar};
//models
mod models;
use models::user::{User, NewUser, LoginUser};
use models::client_to_room::{ClientToRoom,NewClientToRoom};
//schema
mod schema;
use schema::client::{self, first_name, client_password, email};
use schema::client_to_room;
#[database("my_db")]
// create user will be expanded for case in which the user can not be login because they do not have uniq data 
pub struct Db(diesel::PgConnection);
#[post("/sign-up",format = "json", data="<user>")]
async fn create_user(user:Json<NewUser>, conn:Db) -> Json<bool>{
    let user_result: Result<User, Error> = conn.run(move |c| {
        diesel::insert_into(client::table).values(user.into_inner()).get_result::<User>(c)
    }).await;
    match user_result{
        Ok(new_user) =>{
            let new_client_to_room = NewClientToRoom{
                client_id: new_user.client_id, 
                room_id:1,
            };
            let _ =conn.run(move |c| {
                diesel::insert_into(client_to_room::table).values(new_client_to_room).get_result::<ClientToRoom>(c)
            }).await;
            Json(true)
        } ,
        Err(_) => Json(false),
    }
}
//login user take the trys to get the first user if the user does not pass the search than
// return a empty json for not found value  
#[post("/login",format = "json", data="<login_user>")]
async fn login_user( login_user: Json<LoginUser>,conn:Db, jar: &CookieJar<'_>) -> Json<LoginUser>{
    let login_user = login_user.into_inner();
    conn.run(move |conn| {
        client::table::filter(client::table,client_password.eq(login_user.client_password).and(email.eq(login_user.email))).first::<User>(conn)
    }).await.map(|value:User|{
        create_cookie(jar, "user_id".to_string(), value.client_id.to_string());
        Json(LoginUser{
            client_password: value.client_password, email: value.email
        })
    }).unwrap_or(Json(LoginUser{client_password: "fail".to_string(), email: "f".to_string()}))
}
#[post("/admin")]
async fn activate_admin(conn:Db, jar: &CookieJar<'_>) -> Json<bool>{
    let hold:i32 = jar.get_private("user_id").map( |cookie|  cookie.value().parse().ok()).unwrap().unwrap();
    let user_result:Result<User, Error> = conn.run(move |c| {
        client::table::find(client::table, hold).first::<User>(c)
    }).await;
    match user_result{
        Ok(item) => {
            if(item.admin_privilege == true ){
                create_cookie(jar, "admin_access".to_string(),  "True".to_string());
            }         
            Json(true)},
        Err(_) => Json(false),
    }

}
//setup defualt feature for the websiteew
fn create_cookie(jar: &CookieJar<'_>, key: String  , id : String){
    let cookie = Cookie::build(key, id.to_string()).http_only(true).secure(true);
    jar.add_private(cookie.finish());
}
#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .mount("/", routes![create_user,login_user])
    .mount("admin", routes)
    .mount("/", FileServer::from(relative!("static")))
}
//
//|user| Json(user)