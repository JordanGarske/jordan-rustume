use crate::schema::client_to_room;
use diesel::{Queryable, Insertable };
use serde::{Serialize, Deserialize};
#[derive( Deserialize, Queryable,Insertable)]
#[diesel(table_name = client_to_room)]
pub struct ClientToRoom {
    pub client_room_id: i32,
    pub client_id: i32,
    pub room_id: i32,
    pub delete_privilege: bool,
    pub edit_privilege: bool,
    pub write_privilege: bool,
}
#[derive( Serialize,Deserialize, Queryable,Insertable)]
#[diesel(table_name = client_to_room)]
pub struct NewClientToRoom {
    pub client_id: i32,
    pub room_id: i32,

}