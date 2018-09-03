use actix_web::{
    AsyncResponder,
    HttpRequest, HttpResponse, FutureResponse, Json
};

use db::{AppState};
use futures::{Future};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MembersParams {
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub password: String,
    pub gender: i8,
    pub pic_url: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberPutParams {
    pub member_id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
    pub pic_url: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberDeleteParams {
    pub member_id: i32,
}

pub fn members_post((item, req): (Json<MembersParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(MembersParams {
            name: o.name,
            email: o.email,
            password: o.password,
            gender: o.gender,
            phone: o.phone,
            pic_url: o.pic_url,
        })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(x) => Ok(HttpResponse::Ok().json(x.to_string())),
        })
        .responder()
}

pub fn members_put((item, req): (Json<MemberPutParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(MemberPutParams {
            member_id: o.member_id,
            name: o.name,
            email: o.email,
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
            Err(x) => Ok(HttpResponse::Ok().json(x.to_string())),
        })
        .responder()
}

pub fn members_delete((item, req): (Json<MemberDeleteParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    let o = item.clone();
    req.state().db
        .send(MemberDeleteParams {
            member_id: o.member_id,
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
