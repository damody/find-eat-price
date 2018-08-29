use actix_web::{
    error, AsyncResponder, Error, HttpMessage,
    HttpRequest, HttpResponse
};

use bytes::BytesMut;
use serde_json;
use db::{AppState};
use futures::{Future, Stream};
use models;
use schema;

use std::f64::consts::PI;
const a:f64 = 6378137.0;
const b:f64 = 6356752.314245;
const lon0:f64 = 121.0 * PI / 180.0;
const k0:f64 = 0.9999;
const dx:f64 = 250000.0;

pub fn lonlat_To_twd97(lon:f64, lat:f64) ->(f64, f64) {
    let lon = (lon/180.0) * PI;
    let lat = (lat/180.0) * PI;
    
    //---------------------------------------------------------
    let e:f64 = (1.0 - b.powf(2.0) / a.powf(2.0)).powf(0.5);
    let e2:f64 = e.powf(2.0)/(1.0 - e.powf(2.0)); 
    let n:f64 = ( a - b ) / ( a + b );
    let nu:f64 = a / ((1.0 - e.powf(2.0) * lat.sin().powf(2.0))).powf(0.5);
    let p:f64 = lon - lon0;
    let A:f64 = a * (1.0 - n + (5.0/4.0) * (n.powf(2.0) - n.powf(3.0)) + (81.0/64.0) * (n.powf(4.0)  - n.powf(5.0)));
    let B:f64 = (3.0 * a * n/2.0) * (1.0 - n + (7.0/8.0)*(n.powf(2.0) - n.powf(3.0)) + (55.0/64.0)*(n.powf(4.0) - n.powf(5.0)));
    let C:f64 = (15.0 * a * (n.powf(2.0))/16.0)*(1.0 - n + (3.0/4.0)*(n.powf(2.0) - n.powf(3.0)));
    let D:f64 = (35.0 * a * (n.powf(3.0))/48.0)*(1.0 - n + (11.0/16.0)*(n.powf(2.0) - n.powf(3.0)));
    let E:f64 = (315.0 * a * (n.powf(4.0))/51.0)*(1.0 - n);
    
    let S:f64 = A * lat - B * (2.0 * lat).sin() +C * (4.0 * lat).sin() - D * (6.0 * lat).sin() + E * (8.0 * lat).sin();

    //計算Y值
    let K1 = S*k0;
    let K2 = k0*nu*(2.0*lat).sin()/4.0;
    let K3 = (k0*nu*lat.sin()*lat.cos().powf(3.0)/24.0) * (5.0 - lat.tan().powf(2.0) + 9.0 * e2 * lat.cos().powf(2.0) + 4.0*(e2.powf(2.0))*(lat.cos().powf(4.0)));        
    let y = K1 + K2*p.powf(2.0) + K3*p.powf(4.0);

    //計算X值
    let K4 = k0*nu*lat.cos();
    let K5 = (k0*nu*lat.cos().powf(3.0)/6.0) * (1.0 - lat.tan().powf(2.0) + e2*(lat.cos().powf(2.0)));
    let x = K4 * p + K5 * p.powf(3.0) + dx;

    (x, y)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeoParams {
    pub lng: f64,
    pub lat: f64,
}


const MAX_SIZE: usize = 256;

pub fn wgs84_to_twd97(req: &HttpRequest<AppState>) -> Box<Future<Item = HttpResponse, Error = Error>> {
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
            let o = serde_json::from_slice::<GeoParams>(&body);
            if let Err(x) = o {
                return Ok(HttpResponse::Ok().json(x.to_string()))
            };
            let o:GeoParams = o.unwrap();
            let (lng, lat) = lonlat_To_twd97(o.lng, o.lat);
            Ok(HttpResponse::Ok().json(GeoParams{lng:lng, lat:lat}))
        })
    .responder()
}
