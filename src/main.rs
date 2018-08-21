//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate bytes;
extern crate dotenv;

extern crate chrono;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use actix::prelude::*;
use actix_web::{
    error, http, middleware, server, App, AsyncResponder, Error, HttpMessage,
HttpRequest, HttpResponse, Json,
};

use bytes::BytesMut;
use futures::{Future, Stream};


use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;


mod db;
mod models;
mod schema;

use db::{CreateMember, DbExecutor};

/// State with DbExecutor address
pub struct AppState {
    db: Addr<DbExecutor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MembersParams {
    pub email: String,
    pub name: String,
    pub phone_number: String,
    pub password: String,
    pub pic_url: Vec<String>,
}
const MAX_SIZE: usize = 262_144; // max payload size is 256k
/// Async request handler
pub fn members_post(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    // HttpRequest::payload() is stream of Bytes objects
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
        .and_then(|body| {
            // body is loaded, now we can deserialize serde-json
            let obj = serde_json::from_slice::<MembersParams>(&body)?;
            Ok(HttpResponse::Ok().json(obj)) // <- send response
        })
    .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    use std::env;
    let sys = actix::System::new("diesel-example");
    let _ = dotenv::dotenv();
    let url = env::var("MYSQL_UNIT_TEST_DATABASE_URL")
        .or_else(|_| env::var("MYSQL_DATABASE_URL"))
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run unit tests");
    // Start 3 db executor actors
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/members", |r| r.method(http::Method::POST).f(members_post))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
