use actix_web::{
    error, http, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse
};

use bytes::BytesMut;
use serde_json;
use db::{CreateMember, AppState};
use futures::{Future, Stream};
use models;
use schema;
use diesel;
use diesel::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MembersParams {
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub gender: i8,
    pub pic_url: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MembersPutParams {
    pub member_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
    pub pic_url: Option<Vec<String>>,
}

use super::schema::member;
#[derive(Deserialize, AsChangeset)]
#[table_name = "member"]
pub struct MembersPut1 {
    pub member_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
}


const MAX_SIZE: usize = 262_144; // max payload size is 256k
/// Async request handler
pub fn members_post(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    // HttpRequest::payload() is stream of Bytes objects
    let db = req.state().db.clone();
    match *req.method() {
        http::Method::GET => println!("get"),
        http::Method::POST => println!("post"),
        http::Method::PUT => println!("put"),
        _ => println!("other"),
    };
    req.payload()
        // `Future::from_err` acts like `?` in that it coerces the error type from
        // the future into the final error type
        .from_err()
        // `fold` will asynchronously read each chunk of the request body and
        // call supplied closure, then it resolves to result of closure
        .fold(BytesMut::new(), move |mut body, chunk| {
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                Err(error::ErrorBadRequest("overflow"))
            } else {
                body.extend_from_slice(&chunk);
                Ok(body)
            }
        })
        // `Future::and_then` can be used to merge an asynchronous workflow with a
        // synchronous workflow
        .and_then(move |body| {
            use self::schema::member::dsl::*;
            use diesel::result::Error;
            
            // body is loaded, now we can deserialize serde-json
            let o:MembersParams = serde_json::from_slice::<MembersParams>(&body)?;
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let mut new_user = models::NewMember {
                email: o.email,
                name: o.name,
                password: o.password,
                gender: o.gender,
                phone_number: "".to_string(),
            };
            if let Some(x) = &o.phone_number {
                new_user.phone_number = x.clone();
            };
            let data = conn.transaction::<models::Member, Error, _>(|| {
                diesel::insert_into(member).values(&new_user).execute(&conn)?;
                member.order(member_id.desc()).first(&conn)
            });
            let o:MembersParams = serde_json::from_slice::<MembersParams>(&body)?;
            /* r2d2 fail so comment
            db.do_send(CreateMember {
                name: o.name,
                email: o.email,
                password: o.password,
                gender: o.gender,
                phone_number: None,
            });*/
            match data {
                Ok(x) => Ok(HttpResponse::Ok().json(x)),
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : "insert fail, maybe email has used.".to_string()}))
            }
        })
    .responder()
}

pub fn members_put(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let db = req.state().db.clone();
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
            use self::schema::member::dsl::*;
            use diesel::result::Error;
            let o:MembersPutParams = serde_json::from_slice::<MembersPutParams>(&body)?;
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let mid = o.member_id.clone();
            let new_user = MembersPut1 {
                member_id: o.member_id,
                name: o.name,
                enable: o.enable,
                gender: o.gender,
                phone_number: o.phone_number,
                password: o.password,
                member_level: o.member_level,
            };
            let data = conn.transaction::<models::Member, Error, _>(|| {
                diesel::update(member.find(mid)).set(&new_user).execute(&conn)?;
                member.find(mid).first(&conn)
            });
            match data {
                Ok(x) => Ok(HttpResponse::Ok().json(x)),
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : "update fail.".to_string()}))
            }
        })
    .responder()
}

pub fn members_delete(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let db = req.state().db.clone();
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
            use self::schema::member::dsl::*;
            use diesel::result::Error;
            let o:MembersPutParams = serde_json::from_slice::<MembersPutParams>(&body)?;
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let mid = o.member_id.clone();
            let res = diesel::delete(member.find(mid)).execute(&conn);
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
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : "delete fail.".to_string()}))
            }
        })
    .responder()
}
