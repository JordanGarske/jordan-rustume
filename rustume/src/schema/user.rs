use diesel::{Queryable, Insertable };
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Queryable,Insertable)]
#[table_name="client"]
pub struct User{
    pub id:i32,
    pub first_name: String,
    pub last_name: String,
    pub admin_id: i32,
}