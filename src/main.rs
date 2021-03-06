//! Actix web diesel example
//!
//! Diesel does not support tokio, so we have to run it in separate threads.
//! Actix supports sync actors by default, so we going to create sync actor
//! that use diesel. Technically sync actors are worker style actors, multiple
//! of them can run in parallel and process messages from same queue.
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actix;
extern crate actix_web;
extern crate futures;
extern crate r2d2;
extern crate uuid;
extern crate bytes;
extern crate dotenv;
extern crate chrono;
extern crate r2d2_diesel;
extern crate mercator;

use actix::prelude::*;
use actix_web::{
    http, middleware, server, App, Error,
    HttpRequest, HttpResponse, pred,
    http::{header},
    middleware::cors::Cors,
};
//use actix_web::{ error, AsyncResponder, HttpMessage, };
//use bytes::BytesMut;
//use futures::{Future, Stream};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

mod db;
mod models;
mod schema;
mod member;
mod food;
mod restaurant;
mod geo_convert;

use member::*;
use restaurant::*;
use food::*;
use db::{DbExecutor, AppState};

/// 404 handler
fn p404(_req: &HttpRequest<AppState>) -> actix_web::Result<actix_web::fs::NamedFile> {
    Ok(actix_web::fs::NamedFile::open("static/404.html")?.set_status_code(http::StatusCode::NOT_FOUND))
}

fn main() -> Result<(), Box<Error>> {
    ::std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    use std::env;
    let sys = actix::System::new("diesel-example");
    let _ = dotenv::dotenv();
    let url = env::var("MYSQL_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set in order to run unit tests");
    // Start 3 db executor actors
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = r2d2::Pool::new(manager)
        .expect("Failed to create pool.");
    let addr = SyncArbiter::start(8, move || DbExecutor(pool.clone()));
    // Start http server
    server::new(move || {
        App::with_state(AppState{db: addr.clone()})
            // enable logger
            .middleware(middleware::Logger::default())
            .configure(|app| {
                Cors::for_app(app)
                    .allowed_origin("http://localhost:4200")
                    //.allowed_origin("https://www.find-eat-price.com")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .resource("/member", |r| {
                        r.post().with(member_post);
                        r.put().with(member_put);
                        r.delete().with(member_delete);
                    })
                    .resource("/restaurant", |r| {
                        r.post().with(restaurant_post);
                        r.put().with(restaurant_put);
                        r.delete().with(restaurant_delete);
                    })
                    .resource("/restaurant/search", |r| {
                        r.post().with(restaurant_search);
                    })
                    .resource("/restaurant/keyword", |r| {
                        r.post().with(restaurant_keyword);
                    })
                    .resource("/food", |r| {
                        r.post().with(food_post);
                        r.put().with(food_put);
                        r.delete().with(food_delete);
                    })
                    .resource("/food/search", |r| {
                        r.post().with(food_search);
                    })
                    .resource("/food/keyword", |r| {
                        r.post().with(food_keyword);
                    })
                    .resource("/food/menu", |r| {
                        r.post().with(food_menu);
                    })
                    .resource("/wgs84_to_twd97", |r| {
                        r.post().f(geo_convert::wgs84_to_twd97);
                    })
                    .resource("/wgs84_to_2degree_zone", |r| {
                        r.post().f(geo_convert::wgs84_to_2degree_zone);
                    })
                    .resource("/wgs84_to_3degree_zone", |r| {
                        r.post().f(geo_convert::wgs84_to_3degree_zone);
                    })
                    .resource("/wgs84_to_6degree_zone", |r| {
                        r.post().f(geo_convert::wgs84_to_6degree_zone);
                    })
                    
                    .register()
            })
            .default_resource(|r| {
                        // 404 for GET request
                        r.method(http::Method::GET).f(p404);
                        // all requests that are not `GET`
                        r.route().filter(pred::Not(pred::Get())).f(
                            |_req| HttpResponse::MethodNotAllowed());
                    })
            
    }).bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
    Ok(())
}
