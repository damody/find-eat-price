
extern crate chrono;
use chrono::NaiveDateTime;
use super::schema::member;
use super::schema::restaurant;
use super::schema::menu;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub error: String,
}

#[derive(Serialize, Queryable)]
pub struct Member {
    pub member_email: String,
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
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "member"]
pub struct NewMember {
    pub member_email: String,
    pub name: String,
    pub phone: String,
    pub password: String,
    pub gender: i8,
}

#[derive(Serialize, Queryable)]
pub struct Restaurant {
    pub restaurant_id: String,
    pub author_id: String,
    pub chain_id: String,
    pub menu_id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub enable: i8,
    pub good: i32,
    pub bad: i32,
    pub open_time: String,
    pub close_time: String,
    pub lng: f32,
    pub lat: f32,
    pub twd97x: f32,
    pub twd97y: f32,
    pub pic_urls: String,
    pub input_date: NaiveDateTime,
}

#[derive(Serialize, Queryable)]
pub struct RestaurantSearchRes {
    pub restaurant_id: String,
    pub chain_id: String,
    pub name: String,
    pub menu_id: String,
    pub good: i32,
    pub bad: i32,
    pub open_time: String,
    pub close_time: String,
    pub lng: f32,
    pub lat: f32,
    pub twd97x: f32,
    pub twd97y: f32,
    pub distance: f32,
    pub pic_urls: String,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "restaurant"]
pub struct NewRestaurant {
    pub restaurant_id: String,
    pub author_id: String,
    pub chain_id: Option<String>,
    pub menu_id: Option<String>,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub lng: f32,
    pub lat: f32,
    pub twd97x: f32,
    pub twd97y: f32,
    pub pic_urls: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "restaurant"]
pub struct RestaurantUpdate {
    pub restaurant_id: String,
    pub chain_id: Option<String>,
    pub menu_id: Option<String>,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub lng: Option<f32>,
    pub lat: Option<f32>,
    pub twd97x: Option<f32>,
    pub twd97y: Option<f32>,
    pub pic_urls: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct Menu {
    pub menu_id: String,
    pub pic_urls: String,
    pub input_date: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "menu"]
pub struct NewMenu {
    pub menu_id: String,
    pub pic_urls: Option<String>,
}
