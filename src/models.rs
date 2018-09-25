
extern crate chrono;
use super::schema::member;
use super::schema::restaurant;
use super::schema::menu;
use super::schema::food;

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
    pub join_date: chrono::NaiveDateTime,
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
    pub restaurant_id: i32,
    pub author_email: String,
    pub chain_id: i32,
    pub menu_id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub enable: i8,
    pub good: i32,
    pub bad: i32,
    pub open_time: String,
    pub close_time: String,
    pub delivery: String,
    pub lng: f32,
    pub lat: f32,
    pub twd97x: f32,
    pub twd97y: f32,
    pub pic_urls: String,
    pub input_date: chrono::NaiveDateTime,
}

#[derive(Serialize, Queryable)]
pub struct RestaurantSearchRes {
    pub restaurant_id: i32,
    pub chain_id: i32,
    pub name: String,
    pub menu_id: i32,
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
    pub author_email: String,
    pub chain_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub delivery: Option<String>,
    pub lng: f32,
    pub lat: f32,
    pub twd97x: f32,
    pub twd97y: f32,
    pub pic_urls: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "restaurant"]
pub struct RestaurantUpdate {
    pub restaurant_id: i32,
    pub chain_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub delivery: Option<String>,
    pub lng: Option<f32>,
    pub lat: Option<f32>,
    pub twd97x: Option<f32>,
    pub twd97y: Option<f32>,
    pub pic_urls: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct Menu {
    pub menu_id: i32,
    pub pic_urls: String,
    pub input_date: chrono::NaiveDateTime,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "menu"]
pub struct NewMenu {
    pub pic_urls: Option<String>,
}

#[derive(Serialize, Queryable)]
pub struct Food {
    pub food_id: i32,
    pub menu_id: i32,
    pub food_name: String,
    pub price: f32,
    pub pic_urls: String,
    pub input_date: chrono::NaiveDateTime,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "food"]
pub struct FoodUpdate {
    pub food_name: Option<String>,
    pub price: Option<f32>,
    pub pic_urls: Option<String>,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "food"]
pub struct NewFood {
    pub menu_id: i32,
    pub food_name: String,
    pub price: f32,
    pub pic_urls: String,
}

#[derive(Serialize, Queryable)]
pub struct FoodSearchRes {
    pub restaurant_id: i32,
    pub restaurant_name: String,
    pub distance: f32,
    pub food_id: i32,
    pub food_name: String,
    pub pic_urls: String,
}