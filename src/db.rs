//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
//use r2d2_diesel::ConnectionManager;
//use r2d2::Pool;
use member;
use restaurant;
use food;
use models;
use schema;
use mercator;
use uuid;
use std::collections::HashSet;

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub DBPool);

/// State with DbExecutor address
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Message for member::MemberParams {
    type Result = Result<models::Member, Error>;
}
impl Handler<member::MemberParams> for DbExecutor {
    type Result = Result<models::Member, Error>;

    fn handle(&mut self, msg: member::MemberParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        info!("{:?}", msg);
        let new_user = models::NewMember {
            member_email: msg.member_email,
            name: msg.name,
            password: msg.password,
            gender: msg.gender,
            phone: msg.phone.unwrap_or("".to_string()),
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(member).values(&new_user).execute(conn)?;
            member.order(member_email.desc()).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for member::MemberPutParams {
    type Result = Result<models::Member, Error>;
}
impl Handler<member::MemberPutParams> for DbExecutor {
    type Result = Result<models::Member, Error>;

    fn handle(&mut self, msg: member::MemberPutParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        info!("{:?}", msg);
        let mid = msg.member_email.clone();
        let new_user = models::MemberUpdate {
            name: msg.name,
            gender: msg.gender,
            enable: msg.enable,
            phone: msg.phone,
            password: msg.password,
            member_level: msg.member_level,
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::update(member.find(mid.clone())).set(&new_user).execute(conn)?;
            member.find(mid.clone()).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for member::MemberDeleteParams {
    type Result = Result<(), Error>;
}
impl Handler<member::MemberDeleteParams> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: member::MemberDeleteParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        info!("{:?}", msg);
        let mid = msg.member_email.clone();
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let res = diesel::delete(member.find(mid)).execute(conn);
        match res {
            Ok(x) => {
                if x == 1 {
                    Ok(())
                } else {
                    Err(error::ErrorInternalServerError("item not found.".to_string()))
                }    
            },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for restaurant::RestaurantParams {
    type Result = Result<models::Restaurant, Error>;
}
impl Handler<restaurant::RestaurantParams> for DbExecutor {
    type Result = Result<models::Restaurant, Error>;

    fn handle(&mut self, msg: restaurant::RestaurantParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::menu::dsl as menu_dsl;
        info!("{:?}", msg);
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_menu = models::NewMenu {
            menu_id: uuid.clone(),
            pic_urls: None,
        };
        let tmenu:Result<models::Menu, Error> = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(menu_dsl::menu).values(&new_menu).execute(conn)?;
            menu_dsl::menu.find(uuid.clone()).first(conn)
        });
        
        match tmenu {
            Ok(tm) => {
                use self::schema::restaurant::dsl as restaurant_dsl;
                let lng = msg.lng;
                let lat = msg.lat;
                let (x,y) = mercator::wgs84_to_twd97(lng as f64, lat as f64);
                let uuid = format!("{}", uuid::Uuid::new_v4());
                let new_user = models::NewRestaurant {
                    restaurant_id: uuid.clone(),
                    author_email: msg.author_email,
                    name: msg.name,
                    phone: msg.phone,
                    email: msg.email,
                    chain_id: msg.chain_id,
                    menu_id: Some(tm.menu_id),
                    open_time: msg.open_time,
                    close_time: msg.close_time,
                    lng: lng,
                    lat: lat,
                    twd97x: x as f32,
                    twd97y: y as f32,
                    pic_urls: Some(json!(msg.pic_urls.unwrap_or(vec![])).to_string()),
                };
                let data:Result<models::Restaurant, Error> = conn.transaction::<_, Error, _>(|| {
                    diesel::insert_into(restaurant_dsl::restaurant).values(&new_user).execute(conn)?;
                    restaurant_dsl::restaurant.find(uuid.clone()).first(conn)
                });
                match data {
                    Ok(r) => {
                        Ok(r)
                    },
                    Err(x) => Err(error::ErrorInternalServerError(x))
                }
            },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }        
    }
}


impl Message for restaurant::RestaurantPutParams {
    type Result = Result<models::Restaurant, Error>;
}
impl Handler<restaurant::RestaurantPutParams> for DbExecutor {
    type Result = Result<models::Restaurant, Error>;

    fn handle(&mut self, msg: restaurant::RestaurantPutParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl as restaurant_dsl;
        info!("{:?}", msg);
        let mid = msg.restaurant_id.clone();
        let new_user = if msg.lng.is_some() && msg.lat.is_some() {
            let lng:f32 = msg.lng.unwrap();
            let lat:f32 = msg.lat.unwrap();
            let (twd97x,twd97y) = mercator::wgs84_to_twd97(lng as f64, lat as f64);
            models::RestaurantUpdate {
                restaurant_id: msg.restaurant_id,
                name: msg.name,
                phone: msg.phone,
                email: msg.email,
                enable: msg.enable,
                chain_id: msg.chain_id,
                menu_id: msg.menu_id,
                open_time: msg.open_time,
                close_time: msg.close_time,
                lng: msg.lng,
                lat: msg.lat,
                twd97x: Some(twd97x as f32),
                twd97y: Some(twd97y as f32),
                pic_urls: Some(json!(msg.pic_urls.unwrap_or(vec![])).to_string()),
            }
        } else {
            models::RestaurantUpdate {
                restaurant_id: msg.restaurant_id,
                name: msg.name,
                phone: msg.phone,
                email: msg.email,
                enable: msg.enable,
                chain_id: msg.chain_id,
                menu_id: msg.menu_id,
                open_time: msg.open_time,
                close_time: msg.close_time,
                lng: None,
                lat: None,
                twd97x: None,
                twd97y: None,
                pic_urls: Some(json!(msg.pic_urls.unwrap_or(vec![])).to_string()),
            }
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::update(restaurant_dsl::restaurant.find(mid.clone())).set(&new_user).execute(conn)?;
            restaurant_dsl::restaurant.find(mid.clone()).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for restaurant::RestaurantDeleteParams {
    type Result = Result<(), Error>;
}
impl Handler<restaurant::RestaurantDeleteParams> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: restaurant::RestaurantDeleteParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl::*;
        info!("{:?}", msg);
        let mid = msg.restaurant_id.clone();
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let res = diesel::delete(restaurant.find(mid)).execute(conn);
        match res {
            Ok(x) => {
                if x == 1 {
                    Ok(())
                } else {
                    Err(error::ErrorInternalServerError("item not found.".to_string()))
                }    
            },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

fn length(x1:f32, y1:f32, x2:f32, y2:f32) -> f32 {
    let x = x1-x2;
    let y = y1-y2;
    return (x*x+y*y).sqrt();
}

impl Message for restaurant::RestaurantSearchParams {
    type Result = Result<Vec<models::RestaurantSearchRes>, Error>;
}
impl Handler<restaurant::RestaurantSearchParams> for DbExecutor {
    type Result = Result<Vec<models::RestaurantSearchRes>, Error>;

    fn handle(&mut self, msg: restaurant::RestaurantSearchParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl as restaurant_dsl;
        info!("{:?}", msg);
        
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let mut data = restaurant_dsl::restaurant.into_boxed();
        data = data.filter(restaurant_dsl::enable.eq(1));
        if msg.name.is_some() {
            data = data.filter(restaurant_dsl::name.like(format!("%{}%", msg.name.unwrap())));
        };
        let mut x:f32 = 0.0;
        let mut y:f32 = 0.0;
        if msg.lng.is_some() && msg.lat.is_some() && msg.range.is_some() {
            let lng = msg.lng.unwrap();
            let lat = msg.lat.unwrap();
            let range = msg.range.unwrap();
            let (xx,yy) = mercator::wgs84_to_twd97(lng as f64, lat as f64);
            x = xx as f32;
            y = yy as f32;

            data = data.filter(restaurant_dsl::twd97x.between(x-range*0.5f32, x+range*0.5f32));
            data = data.filter(restaurant_dsl::twd97y.between(y-range*0.5f32, y+range*0.5f32));
        };
        if msg.like.is_some() {
            data = data.filter(restaurant_dsl::good.ge(msg.like.unwrap()));
        }
        if msg.dislike.is_some() {
            data = data.filter(restaurant_dsl::bad.le(msg.dislike.unwrap()));
        }
        let data = data.load::<models::Restaurant>(conn);
        
        match data {
            Ok(defd) => {
                let res = defd.into_iter().map(move |v:models::Restaurant| {
                    models::RestaurantSearchRes {
                        restaurant_id: v.restaurant_id,
                        chain_id: v.chain_id,
                        name: v.name,
                        good: v.good,
                        bad: v.bad,
                        menu_id: v.menu_id,
                        open_time: v.open_time,
                        close_time: v.close_time,
                        lng: v.lng,
                        lat: v.lat,
                        twd97x: v.twd97x,
                        twd97y: v.twd97y,
                        distance: length(v.twd97x, v.twd97y, x, y),
                        pic_urls: v.pic_urls,
                    }
                }).rev().collect();
                Ok(res)
                },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for food::FoodParams {
    type Result = Result<models::Food, Error>;
}
impl Handler<food::FoodParams> for DbExecutor {
    type Result = Result<models::Food, Error>;

    fn handle(&mut self, msg: food::FoodParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::food::dsl as food_dsl;
        info!("{:?}", msg);
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_food = models::NewFood {
            food_id: uuid.clone(),
            menu_id: msg.menu_id,
            food_name: msg.food_name,
            price: msg.price,
            pic_urls: json!(msg.pic_urls.unwrap_or(vec![])).to_string(),
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(food_dsl::food).values(&new_food).execute(conn)?;
            food_dsl::food.find(uuid.clone()).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for food::FoodPutParams {
    type Result = Result<models::Food, Error>;
}
impl Handler<food::FoodPutParams> for DbExecutor {
    type Result = Result<models::Food, Error>;

    fn handle(&mut self, msg: food::FoodPutParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::food::dsl as food_dsl;
        info!("{:?}", msg);
        let mid = msg.food_id;
        let new_food = models::FoodUpdate {
            food_name: msg.food_name,
            price: msg.price,
            pic_urls: Some(json!(msg.pic_urls.unwrap_or(vec![])).to_string()),
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::update(food_dsl::food.find(mid.clone())).set(&new_food).execute(conn)?;
            food_dsl::food.find(mid.clone()).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for food::FoodDeleteParams {
    type Result = Result<(), Error>;
}
impl Handler<food::FoodDeleteParams> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: food::FoodDeleteParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::food::dsl as food_dsl;
        info!("{:?}", msg);
        let mid = msg.food_id;
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let res = diesel::delete(food_dsl::food.find(mid)).execute(conn);
        match res {
            Ok(x) => {
                if x == 1 {
                    Ok(())
                } else {
                    Err(error::ErrorInternalServerError("item not found.".to_string()))
                }    
            },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for food::FoodSearchParams {
    type Result = Result<Vec<models::FoodSearchRes>, Error>;
}
impl Handler<food::FoodSearchParams> for DbExecutor {
    type Result = Result<Vec<models::FoodSearchRes>, Error>;

    fn handle(&mut self, msg: food::FoodSearchParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl as restaurant_dsl;
        use self::schema::food::dsl as food_dsl;
        info!("{:?}", msg);
        
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let mut data = food_dsl::food
                .inner_join(restaurant_dsl::restaurant.on(
                    food_dsl::menu_id.eq(restaurant_dsl::menu_id)
                )).into_boxed();
        data = data.filter(restaurant_dsl::enable.eq(1));
        if msg.food_name.is_some() {
            data = data.filter(food_dsl::food_name.like(format!("%{}%", msg.food_name.unwrap())));
        };
        let mut x:f32 = 0.0;
        let mut y:f32 = 0.0;
        if msg.lng.is_some() && msg.lat.is_some() && msg.range.is_some() {
            let lng = msg.lng.unwrap();
            let lat = msg.lat.unwrap();
            let range = msg.range.unwrap();
            let (xx,yy) = mercator::wgs84_to_twd97(lng as f64, lat as f64);
            x = xx as f32;
            y = yy as f32;

            data = data.filter(restaurant_dsl::twd97x.between(x-range*0.5f32, x+range*0.5f32));
            data = data.filter(restaurant_dsl::twd97y.between(y-range*0.5f32, y+range*0.5f32));
        };
        if msg.like.is_some() {
            data = data.filter(restaurant_dsl::good.ge(msg.like.unwrap()));
        }
        if msg.dislike.is_some() {
            data = data.filter(restaurant_dsl::bad.le(msg.dislike.unwrap()));
        }
        let data = data.load::<(models::Food, models::Restaurant)>(conn);
        
        match data {
            Ok(defd) => {
                let res = defd.into_iter().map(move |(f,r):(models::Food, models::Restaurant)| {
                    models::FoodSearchRes {
                        restaurant_id: r.restaurant_id,
                        restaurant_name: r.name,
                        distance: length(r.twd97x, r.twd97y, x, y),
                        food_id: f.food_id,
                        food_name: f.food_name,
                        pic_urls: f.pic_urls,
                    }
                }).rev().collect();
                Ok(res)
                },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for food::FoodKeywordParams {
    type Result = Result<Vec<String>, Error>;
}
impl Handler<food::FoodKeywordParams> for DbExecutor {
    type Result = Result<Vec<String>, Error>;
    fn handle(&mut self, msg: food::FoodKeywordParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::food::dsl as food_dsl;
        info!("{:?}", msg);
        
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let mut data = food_dsl::food.into_boxed();
        if msg.fuzzy {
            for name in msg.food_name.split_whitespace() {
                data = data.filter(food_dsl::food_name.like(format!("%{}%", name)));
            }
        } else {
            data = data.filter(food_dsl::food_name.like(format!("{}%", msg.food_name)));
        }
        let data = data.load::<models::Food>(conn);
        
        match data {
            Ok(defd) => {
                let mut res:Vec<String> = defd.into_iter().map(move |f:models::Food| {
                    f.food_name
                }).rev().collect();
                let set: HashSet<_> = res.drain(..).collect(); // dedup
                res.extend(set.into_iter());
                Ok(res)
                },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for food::FoodMenuParams {
    type Result = Result<Vec<models::Food>, Error>;
}
impl Handler<food::FoodMenuParams> for DbExecutor {
    type Result = Result<Vec<models::Food>, Error>;

    fn handle(&mut self, msg: food::FoodMenuParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::food::dsl as food_dsl;
        info!("{:?}", msg);
        
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let mut data = food_dsl::food.into_boxed();
        data = data.filter(food_dsl::menu_id.eq(msg.menu_id));
        let data = data.load::<models::Food>(conn);
        
        match data {
            Ok(defd) => {
                Ok(defd)
                },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for restaurant::RestaurantKeywordParams {
    type Result = Result<Vec<String>, Error>;
}
impl Handler<restaurant::RestaurantKeywordParams> for DbExecutor {
    type Result = Result<Vec<String>, Error>;
    fn handle(&mut self, msg: restaurant::RestaurantKeywordParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl as restaurant_dsl;
        info!("{:?}", msg);
        
        let conn: &MysqlConnection = &self.0.get().unwrap();
        let mut data = restaurant_dsl::restaurant.into_boxed();
        if msg.fuzzy {
            for name in msg.name.split_whitespace() {
                data = data.filter(restaurant_dsl::name.like(format!("%{}%", name)));
            }
        } else {
            data = data.filter(restaurant_dsl::name.like(format!("{}%", msg.name)));
        }
        let data = data.load::<models::Restaurant>(conn);
        
        match data {
            Ok(defd) => {
                let mut res:Vec<String> = defd.into_iter().map(move |f:models::Restaurant| {
                    f.name
                }).rev().collect();
                let set: HashSet<_> = res.drain(..).collect(); // dedup
                res.extend(set.into_iter());
                Ok(res)
                },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}
