use crate::schema::client_to_room;
use diesel::{Queryable, Insertable };
use serde::{Serialize, Deserialize};
#[derive( Deserialize, Queryable,Insertable)]
#[diesel(table_name = client_to_room)]
pub struct ClientToRoom {
    pub room_id: i32,
    pub title: String,
    pub elucidation: String,
}
