
extern crate chrono;
use chrono::NaiveDateTime;
use super::schema::member;

#[derive(Serialize, Queryable)]
pub struct Member {
    pub member_id: i32,
    pub email: String,
    pub name: String,
    pub enable: i8,
    pub gender: i8,
    pub phone_number: String,
    pub password: String,
    pub member_level: i8,
    pub join_date: NaiveDateTime,
}

#[derive(Deserialize, Insertable, Default)]
#[table_name = "member"]
pub struct NewMember {
    pub email: String,
    pub name: String,
    pub phone_number: String,
    pub password: String,
    pub gender: i8,
}
