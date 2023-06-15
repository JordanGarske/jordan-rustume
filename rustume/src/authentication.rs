use crate::models::client_to_room::{NewClientToRoom, ClientToRoom};
use crate::models::user::{NewUser, LoginUser, User};
use crate::schema::{client, client_to_room};
use rocket::{serde::json::Json, http::CookieJar};
use crate::{Db, create_cookie};
//diesel 
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::schema::client::{  client_password, email};
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

pub fn routes() -> Vec<rocket::Route> {
    routes![login_user, create_user]
}














