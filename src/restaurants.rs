use actix_web::{
    error, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse
};

use bytes::BytesMut;
use serde_json;
use db::{AppState};
use futures::{Future, Stream};
use models;
use schema;
use diesel;
use diesel::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantsParams {
    pub author_id: i32,
    pub food_id: Option<i32>,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub chain_id: Option<i32>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub lat: f32,
    pub lng: f32,
    pub pic_url: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantsPutParams {
    pub restaurant_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub pic_url: Option<Vec<String>>,
}

use super::schema::restaurant;
#[derive(Deserialize, AsChangeset)]
#[table_name = "restaurant"]
pub struct RestaurantsPut1 {
    pub restaurant_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
}


const MAX_SIZE: usize = 262_144; // max payload size is 256k

pub fn restaurants_post(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let _db = req.state().db.clone();
    req.payload()
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        .and_then(move |body| {
            use self::schema::restaurant::dsl::*;
            use diesel::result::Error;
            let o = serde_json::from_slice::<RestaurantsParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:RestaurantsParams = o.unwrap();
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let new_rest = models::NewRestaurant {
                email: o.email.unwrap_or("".to_string()),
                name: o.name,
                author_id: o.author_id,
                food_id: o.food_id,
                chain_id: o.chain_id.unwrap_or(-1),
                phone: o.phone.unwrap_or("".to_string()),
                open_time: o.open_time,
                close_time: o.close_time,
            };
            let data = conn.transaction::<models::Restaurant, Error, _>(|| {
                diesel::insert_into(restaurant).values(&new_rest).execute(&conn)?;
                restaurant.order(restaurant_id.desc()).first(&conn)
            });
            match data {
                Ok(x) => Ok(HttpResponse::Ok().json(x)),
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : x.to_string()}))
            }
        })
    .responder()
}

pub fn restaurants_put(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let _db = req.state().db.clone();
    req.payload()
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        .and_then(move |body| {
            use self::schema::restaurant::dsl::*;
            use diesel::result::Error;
            let o = serde_json::from_slice::<RestaurantsPutParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:RestaurantsPutParams = o.unwrap();
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let mid = o.restaurant_id.clone();
            let new_user = RestaurantsPut1 {
                restaurant_id: o.restaurant_id,
                name: o.name,
                enable: o.enable,
                phone: o.phone,
                email: o.email,
                open_time: o.open_time,
                close_time: o.close_time,
            };
            let data = conn.transaction::<models::Restaurant, Error, _>(|| {
                diesel::update(restaurant.find(mid)).set(&new_user).execute(&conn)?;
                restaurant.find(mid).first(&conn)
            });
            match data {
                Ok(x) => Ok(HttpResponse::Ok().json(x)),
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : x.to_string()}))
            }
        })
    .responder()
}

pub fn restaurants_delete(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let _db = req.state().db.clone();
    req.payload()
        .from_err()
        .fold(BytesMut::new(), move |mut body, chunk| {
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        .and_then(move |body| {
            use self::schema::restaurant::dsl::*;
            //use diesel::result::Error;
            let o:RestaurantsPutParams = serde_json::from_slice::<RestaurantsPutParams>(&body)?;
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let mid = o.restaurant_id.clone();
            let res = diesel::delete(restaurant.find(mid)).execute(&conn);
            match res {
                Ok(x) => {
                    if x == 1 {
                        let mut hash = HashMap::new();
                        hash.insert("msg", "ok");
                        Ok(HttpResponse::Ok().json(hash))
                    } else {
                        Ok(HttpResponse::Ok().json(models::ErrorMessage {error : "item not found.".to_string()}))
                    }
                    
                    },
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : x.to_string()}))
            }
        })
    .responder()
}
