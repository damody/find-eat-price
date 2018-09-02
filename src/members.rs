use actix_web::{
    error, http, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse, FutureResponse, Json
};

use bytes::BytesMut;
use serde_json;
use db;
use db::{AppState};
use futures::{Future, Stream};
use models;
use schema;
use diesel;
use diesel::prelude::*;
use std::collections::HashMap;
use futures::future::{join_all, ok as fut_ok};

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
pub struct MembersPutParams {
    pub member_id: i32,
    pub name: Option<String>,
    pub enable: Option<i8>,
    pub gender: Option<i8>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub member_level: Option<i8>,
    pub pic_url: Option<Vec<String>>,
}

pub fn members_post2((item, req): (Json<MembersParams>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
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

const MAX_SIZE: usize = 262_144; // max payload size is 256k
/// Async request handler
pub fn members_post(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            use self::schema::member::dsl::*;
            use diesel::result::Error;
            let o = serde_json::from_slice::<MembersParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:MembersParams = o.unwrap();
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            
            println!("{:?}", o);
            let mut new_user = models::NewMember {
                email: o.email,
                name: o.name,
                password: o.password,
                gender: o.gender,
                phone: "".to_string(),
            };
            if let Some(x) = &o.phone {
                new_user.phone = x.clone();
            };
            
            let data = conn.transaction::<models::Member, Error, _>(|| {
                diesel::insert_into(member).values(&new_user).execute(&conn)?;
                member.order(member_id.desc()).first(&conn)
            });
            let o:MembersParams = serde_json::from_slice::<MembersParams>(&body)?;
            match data {
                Ok(x) => Ok(HttpResponse::Ok().json(x)),
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : x.to_string()}))
            }
        })
    .responder()
}

pub fn members_put(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            use self::schema::member::dsl::*;
            use diesel::result::Error;
            let o = serde_json::from_slice::<MembersPutParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:MembersPutParams = o.unwrap();
            let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
            println!("{:?}", o);
            let mid = o.member_id.clone();
            let new_user = models::MemberUpdate {
                member_id: o.member_id,
                name: o.name,
                enable: o.enable,
                gender: o.gender,
                phone: o.phone,
                password: o.password,
                member_level: o.member_level,
            };
            let data = conn.transaction::<models::Member, Error, _>(|| {
                diesel::update(member.find(mid)).set(&new_user).execute(&conn)?;
                member.find(mid).first(&conn)
            });
            match data {
                Ok(x) => Ok(HttpResponse::Ok().json(x)),
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : x.to_string()}))
            }
        })
    .responder()
}

pub fn members_delete(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            use self::schema::member::dsl::*;
            //use diesel::result::Error;
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
                Err(x) => Ok(HttpResponse::Ok().json(models::ErrorMessage {error : x.to_string()}))
            }
        })
    .responder()
}
