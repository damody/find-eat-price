use actix_web::{
    AsyncResponder,
    HttpRequest, HttpResponse, FutureResponse, Json
};

use db::{AppState};
use futures::{Future};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantParams {
    pub author_id: i32,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub chain_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub lat: f32,
    pub lng: f32,
    pub pic_url: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantPutParams {
    pub restaurant_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub chain_id: Option<i32>,
    pub menu_id: Option<i32>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub pic_url: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantDeleteParams {
    pub restaurant_id: i32,
}

pub fn restaurants_post((item, req): (Json<RestaurantParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(RestaurantParams {
            author_id: o.author_id,
            name: o.name,
            phone: o.phone,
            email: o.email,
            chain_id: o.chain_id,
            menu_id: o.menu_id,
            open_time: o.open_time,
            close_time: o.close_time,
            lat: o.lat,
            lng: o.lng,
            pic_url: o.pic_url,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => Ok(HttpResponse::Ok().json(x.to_string())),
        })
        .responder()
}

pub fn restaurants_put((item, req): (Json<RestaurantPutParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(RestaurantPutParams {
            restaurant_id: o.restaurant_id,
            name: o.name,
            enable: o.enable,
            phone: o.phone,
            email: o.email,
            chain_id: o.chain_id,
            menu_id: o.menu_id,
            open_time: o.open_time,
            close_time: o.close_time,
            pic_url: o.pic_url,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => Ok(HttpResponse::Ok().json(x.to_string())),
        })
        .responder()
}

pub fn restaurants_delete((item, req): (Json<RestaurantDeleteParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(RestaurantDeleteParams {
            restaurant_id: o.restaurant_id,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(_) => {
                let mut hash = HashMap::new();
                hash.insert("msg", "ok");
                Ok(HttpResponse::Ok().json(hash))
            },
            Err(x) => {
                let mut hash = HashMap::new();
                hash.insert("msg", x.to_string());
                Ok(HttpResponse::Ok().json(hash))
            },
        })
        .responder()
}
