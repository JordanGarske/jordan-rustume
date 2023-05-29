
use crate::schema::client;
use diesel::{Queryable, Insertable };
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Queryable,Insertable)]
#[diesel(table_name = client)]
pub struct User{
    pub client_id:i32,
    pub first_name: String,
    pub last_name: String,
    pub client_password: String,
    pub email: String,
    pub admin_privilege:bool

}
#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = client)]
pub struct NewUser{
    pub first_name: String,
    pub last_name: String,
    pub client_password: String,
    pub email: String
}
