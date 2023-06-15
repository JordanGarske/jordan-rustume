use diesel::result::Error;
use rocket::{serde::json::Json, http::CookieJar};
use crate::{Db};
use crate::models::user::{Client};
use crate::models::room::Room;
use crate::models::room_subject::RoomSubject;

use crate::models::client_to_room::NewClientToRoom;
use crate::schema::{room_subject, room};
use diesel::{BelongingToDsl, SelectableHelper};
use diesel::prelude::*;
//-> Json<Room
#[get("/")]
async fn rooms( conn:Db, jar: &CookieJar<'_>) -> Json<Vec<Room>>{
    let hold:i32 = jar.get_private("user_id").map( |cookie|  cookie.value().parse().ok()).unwrap().unwrap();
    let user:Client = Client { client_id: hold };
 
    let result_rooms: Result<Vec<Room>, Error> = conn.run(move |c| {
        NewClientToRoom::belonging_to(&user).inner_join(room::table).select(Room::as_select()).load::<Room>(c)
    }).await;
    match result_rooms{
        Ok(rooms) =>{
            return Json(rooms);
        }
        Err(err) => {
            print!("{}" ,err);
            return Json(vec![Room{room_id:-1, title: "err".to_string(), elucidation: "err".to_string() }])
        }
    }
}
#[get("/<id>")]
async fn sub_rooms(id: i32 ,  conn:Db, jar: &CookieJar<'_>) -> Json<Vec<RoomSubject>>{
    //let hold:i32 = jar.get_private("user_id").map( |cookie|  cookie.value().parse().ok()).unwrap().unwrap(); 
    let result_rooms: Result<Vec<RoomSubject>, Error> = conn.run(move |c| {
        room_subject::table::filter(room_subject::table,room_subject::room_id.eq(id)).load::<RoomSubject>(c)
    }).await;
    match result_rooms{
        Ok(rooms) =>{
            return Json(rooms);
        }
        Err(err) => {
            print!("{}" ,err);
            return Json(vec![RoomSubject{room_id: -1 ,title:"err".to_string(),elucidation:"err".to_string(), subject_id: -1 , client_room_id: -1, subject_type: "err".to_string() }])
        }
    }
}
pub fn routes() -> Vec<rocket::Route> {
    routes![rooms, sub_rooms]
}
