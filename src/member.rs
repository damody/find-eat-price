use actix_web::{
    AsyncResponder,
    HttpRequest, HttpResponse, FutureResponse, Json
};

use db::{AppState};
use futures::{Future};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberParams {
    pub member_email: String,
    pub name: String,
    pub phone: Option<String>,
    pub password: String,
    pub gender: i8,
    pub pic_url: Option<Vec<String>>,
}

pub fn member_post((item, req): (Json<MemberParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(MemberParams {
            member_email: o.member_email,
            name: o.name,
            password: o.password,
            gender: o.gender,
            phone: o.phone,
            pic_url: o.pic_url,
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
pub struct MemberPutParams {
    pub member_email: String,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
    pub pic_url: Option<Vec<String>>,
}

pub fn member_put((item, req): (Json<MemberPutParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(MemberPutParams {
            member_email: o.member_email,
            name: o.name,
            enable: o.enable,
            gender: o.gender,
            phone: o.phone,
            password: o.password,
            member_level: o.member_level,
            pic_url: o.pic_url,
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
pub struct MemberDeleteParams {
    pub member_email: String,
}

pub fn member_delete((item, req): (Json<MemberDeleteParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(MemberDeleteParams {
            member_email: o.member_email,
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
                hash.insert("error", x.to_string());
                Ok(HttpResponse::Ok().json(hash))
            },
        })
        .responder()
}
