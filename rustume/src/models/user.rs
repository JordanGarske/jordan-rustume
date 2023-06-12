use crate::schema::client;
use diesel::{Queryable, Insertable, Identifiable, Selectable };
use serde::{Serialize, Deserialize};

#[derive( Queryable,Insertable)]
#[diesel(table_name = client)]
pub struct User{
    pub client_id:i32,
    pub first_name: String,
    pub last_name: String,
    pub client_password: String,
    pub email: String,
    pub admin_privilege:bool

}
#[derive(Deserialize,Insertable, Queryable)]
#[diesel(table_name = client)]
pub struct NewUser{
    pub first_name: String,
    pub last_name: String,
    pub client_password: String,
    pub email: String
}
#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = client)]
pub struct LoginUser{
    pub client_password: String,
    pub email: String
}
#[derive(Queryable, Identifiable , Selectable)]
#[diesel(table_name = client)]
#[diesel(primary_key(client_id))]
pub struct Client{
    pub client_id: i32,  
}