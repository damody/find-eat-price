
extern crate chrono;
use chrono::NaiveDateTime;
use super::schema::member;
use super::schema::restaurant;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error: String,
}

#[derive(Serialize, Queryable)]
pub struct Member {
    pub member_id: i32,
    pub email: String,
    pub name: String,
    pub enable: i8,
    pub gender: i8,
    pub phone: String,
    pub password: String,
    pub member_level: i8,
    pub join_date: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "member"]
pub struct NewMember {
    pub email: String,
    pub name: String,
    pub phone: String,
    pub password: String,
    pub gender: i8,
}

#[derive(Serialize, Queryable)]
pub struct Restaurant {
    pub restaurant_id: i32,
    pub author_id: i32,
    pub chain_id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub enable: i8,
    pub good: i32,
    pub bad: i32,
    pub menu_id: i32,
    pub open_time: String,
    pub close_time: String,
    pub input_date: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "restaurant"]
pub struct NewRestaurant {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub chain_id: i32,
    pub author_id: i32,
    pub menu_id: Option<i32>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
}
