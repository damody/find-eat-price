use actix_web::{
    AsyncResponder,
    HttpRequest, HttpResponse, FutureResponse, Json
};

use db::{AppState};
use futures::{Future};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantParams {
    pub author_email: String,
    pub chain_id: Option<i32>,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub delivery: Option<String>,
    pub lat: f32,
    pub lng: f32,
    pub pic_urls: Option<Vec<String>>,
}

pub fn restaurant_post((item, req): (Json<RestaurantParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(RestaurantParams {
            author_email: o.author_email,
            name: o.name,
            phone: o.phone,
            email: o.email,
            chain_id: o.chain_id,
            open_time: o.open_time,
            close_time: o.close_time,
            delivery: o.delivery,
            lat: o.lat,
            lng: o.lng,
            pic_urls: o.pic_urls,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => {
                let mut hash = HashMap::new();
                hash.insert("error", x.to_string());
                Ok(HttpResponse::Ok().json(hash))
            },
        })
        .responder()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantPutParams {
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
    pub pic_urls: Option<Vec<String>>,
    pub lng: Option<f32>,
    pub lat: Option<f32>,
}

pub fn restaurant_put((item, req): (Json<RestaurantPutParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
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
            delivery: o.delivery,
            pic_urls: o.pic_urls,
            lng: o.lng,
            lat: o.lat,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => {
                let mut hash = HashMap::new();
                hash.insert("error", x.to_string());
                Ok(HttpResponse::Ok().json(hash))
            },
        })
        .responder()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantDeleteParams {
    pub restaurant_id: i32,
}

pub fn restaurant_delete((item, req): (Json<RestaurantDeleteParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantSearchParams {
    pub name: String,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub range: Option<f32>,
    pub like: Option<i32>,
    pub dislike: Option<i32>,
    pub fuzzy: Option<bool>,
}

pub fn restaurant_search((item, req): (Json<RestaurantSearchParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(RestaurantSearchParams {
            name: o.name,
            lng: o.lng,
            lat: o.lat,
            range: o.range,
            like: o.like,
            dislike: o.dislike,
            fuzzy: o.fuzzy,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => {
                let mut hash = HashMap::new();
                hash.insert("error", x.to_string());
                Ok(HttpResponse::Ok().json(hash))
            },
        })
        .responder()
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestaurantKeywordParams {
    pub name: String,
    pub fuzzy: bool,
}

pub fn restaurant_keyword((item, req): (Json<RestaurantKeywordParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(RestaurantKeywordParams {
            name: o.name,
            fuzzy: o.fuzzy,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => {
                let mut hash = HashMap::new();
                hash.insert("error", x.to_string());
                Ok(HttpResponse::Ok().json(hash))
            },
        })
        .responder()
}

