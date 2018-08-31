use actix_web::{
    error, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse
};

use bytes::BytesMut;
use serde_json;
use db::{AppState};
use futures::{Future, Stream};
use mercator::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoParams {
    pub lng: f64,
    pub lat: f64,
    pub center_lng: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoRes {
    pub x: f64,
    pub y: f64,
}

const MAX_SIZE: usize = 256;

pub fn wgs84_to_twd97(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            let o = serde_json::from_slice::<GeoParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:GeoParams = o.unwrap();
            let k0:f64 = 0.9999;
            let dx:f64 = 250000.0;
            let (x, y) = lnglat_to_mercator(o.lng, o.lat, 121.0, k0, dx);
            //let (x, y) = mercator_to_lnglat(x, y, 121.0, k0, dx);
            Ok(HttpResponse::Ok().json(GeoRes{x:x, y:y}))
        })
    .responder()
}

pub fn wgs84_to_2degree_zone(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            let o = serde_json::from_slice::<GeoParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:GeoParams = o.unwrap();
            let k0:f64 = 0.9999;
            let dx:f64 = 250000.0;
            let (x, y) = lnglat_to_mercator(o.lng, o.lat, o.center_lng.unwrap_or(121.0), k0, dx);
            Ok(HttpResponse::Ok().json(GeoRes{x:x, y:y}))
        })
    .responder()
}

pub fn wgs84_to_3degree_zone(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            let o = serde_json::from_slice::<GeoParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:GeoParams = o.unwrap();
            let k0:f64 = 1.0;
            let dx:f64 = 350000.0;
            let (x, y) = lnglat_to_mercator(o.lng, o.lat, o.center_lng.unwrap_or(121.0), k0, dx);
            Ok(HttpResponse::Ok().json(GeoRes{x:x, y:y}))
        })
    .responder()
}


pub fn wgs84_to_6degree_zone(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            let o = serde_json::from_slice::<GeoParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:GeoParams = o.unwrap();
            let k0:f64 = 0.9996;
            let dx:f64 = 500000.0;
            let (x, y) = lnglat_to_mercator(o.lng, o.lat, o.center_lng.unwrap_or(123.0), k0, dx);
            Ok(HttpResponse::Ok().json(GeoRes{x:x, y:y}))
        })
    .responder()
}
