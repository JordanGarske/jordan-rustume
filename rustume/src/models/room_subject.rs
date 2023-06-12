use crate::schema::room_subject;
use diesel::{Queryable, Insertable, Identifiable , Selectable };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Insertable, Queryable, Identifiable , Selectable)]
#[diesel(table_name = room_subject)]
#[diesel(primary_key(room_id))]
pub struct RoomSubject {
    pub subject_id: i32,
    pub client_room_id: i32,
    pub room_id: i32,
    pub title: String,
    pub elucidation: String,
    pub subject_type: String,
}
