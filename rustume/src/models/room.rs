use crate::schema::room;
use diesel::{Queryable, Insertable, Identifiable , Selectable };
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Insertable, Queryable, Identifiable , Selectable)]
#[diesel(table_name = room)]
#[diesel(primary_key(room_id))]
pub struct Room {
    pub room_id: i32,
    pub title: String,
    pub elucidation: String,
}
