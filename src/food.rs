use actix_web::{
    AsyncResponder,
    HttpRequest, HttpResponse, FutureResponse, Json
};

use db::{AppState};
use futures::{Future};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FoodParams {
    pub menu_id: String,
    pub food_name: String,
    pub price: f32,
    pub pic_urls: Option<Vec<String>>,
}

pub fn food_post((item, req): (Json<FoodParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(FoodParams {
            menu_id: o.menu_id,
            food_name: o.food_name,
            price: o.price,
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
pub struct FoodPutParams {
    pub food_id: String,
    pub food_name: Option<String>,
    pub price: Option<f32>,
    pub pic_urls: Option<Vec<String>>,
}

pub fn food_put((item, req): (Json<FoodPutParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(FoodPutParams {
            food_id: o.food_id,
            food_name: o.food_name,
            price: o.price,
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
pub struct FoodDeleteParams {
    pub food_id: String,
}

pub fn food_delete((item, req): (Json<FoodDeleteParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(FoodDeleteParams {
            food_id: o.food_id,
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
pub struct FoodSearchParams {
    pub food_name: Option<String>,
    pub lat: Option<f32>,
    pub lng: Option<f32>,
    pub range: Option<f32>,
    pub like: Option<i32>,
    pub dislike: Option<i32>,
}

pub fn food_search((item, req): (Json<FoodSearchParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(FoodSearchParams {
            food_name: o.food_name,
            lng: o.lng,
            lat: o.lat,
            range: o.range,
            like: o.like,
            dislike: o.dislike,
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
