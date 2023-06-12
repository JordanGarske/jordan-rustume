use crate::schema::{client_to_room};
use diesel::{Queryable, Insertable, Associations, Identifiable };
use serde::{Serialize, Deserialize};
use crate::models::{user, room};
#[derive( Deserialize,Serialize, Queryable,Insertable)]
#[diesel(table_name = client_to_room)]
pub struct ClientToRoom {
    pub client_room_id: i32,
    pub client_id: i32,
    pub room_id: i32,
    pub delete_privilege: bool,
    pub edit_privilege: bool,
    pub write_privilege: bool,
}
#[derive( Identifiable,Associations,Queryable,Insertable)]
#[diesel(table_name = client_to_room)]
#[diesel(belongs_to(user::Client))]
#[diesel(belongs_to(room::Room))]
#[diesel(primary_key(client_id, room_id))]
pub struct NewClientToRoom {
    pub client_id: i32,
    pub room_id: i32
}


