
extern crate chrono;
use chrono::NaiveDateTime;
use super::schema::member;
use super::schema::menu;
use super::schema::restaurant;
use super::schema::restaurant_pos;

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

#[derive(Deserialize, AsChangeset)]
#[table_name = "member"]
pub struct MemberUpdate {
    pub member_id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
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
    pub author_id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub chain_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "restaurant_pos"]
pub struct NewRestaurantPos {
    pub restaurant_id: i32,
    pub lng: f32,
    pub lat: f32,
    pub twd97x: f32,
    pub twd97y: f32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "restaurant"]
pub struct RestaurantUpdate {
    pub restaurant_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub chain_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct Menu {
    pub menu_id: i32,
    pub restaurant_id: i32,
    pub input_date: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "menu"]
pub struct NewMenu {
    pub restaurant_id: i32,
}
