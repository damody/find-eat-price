//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{Pool};
use r2d2_diesel::ConnectionManager;
//use r2d2_diesel::ConnectionManager;
//use r2d2::Pool;
use members;
use restaurants;
use models;
use schema;

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

impl Message for members::MembersParams {
    type Result = Result<models::Member, Error>;
}
impl Handler<members::MembersParams> for DbExecutor {
    type Result = Result<models::Member, Error>;

    fn handle(&mut self, msg: members::MembersParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        println!("{:?}", msg);
        let new_user = models::NewMember {
            email: msg.email,
            name: msg.name,
            password: msg.password,
            gender: msg.gender,
            phone: msg.phone.unwrap_or("".to_string()),
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(member).values(&new_user).execute(conn)?;
            member.order(member_id.desc()).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for members::MemberPutParams {
    type Result = Result<models::Member, Error>;
}
impl Handler<members::MemberPutParams> for DbExecutor {
    type Result = Result<models::Member, Error>;

    fn handle(&mut self, msg: members::MemberPutParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        println!("{:?}", msg);
        let mid = msg.member_id.clone();
        let new_user = models::MemberUpdate {
            member_id: msg.member_id,
            name: msg.name,
            email: msg.email,
            gender: msg.gender,
            enable: msg.enable,
            phone: msg.phone,
            password: msg.password,
            member_level: msg.member_level,
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::update(member.find(mid)).set(&new_user).execute(conn)?;
            member.find(mid).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for members::MemberDeleteParams {
    type Result = Result<(), Error>;
}
impl Handler<members::MemberDeleteParams> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: members::MemberDeleteParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        println!("{:?}", msg);
        let mid = msg.member_id.clone();
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

impl Message for restaurants::RestaurantParams {
    type Result = Result<models::Restaurant, Error>;
}
impl Handler<restaurants::RestaurantParams> for DbExecutor {
    type Result = Result<models::Restaurant, Error>;

    fn handle(&mut self, msg: restaurants::RestaurantParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl::*;
        println!("{:?}", msg);
        let new_user = models::NewRestaurant {
            author_id: msg.author_id,
            name: msg.name,
            phone: msg.phone,
            email: msg.email,
            chain_id: msg.chain_id,
            menu_id: msg.menu_id,
            open_time: msg.open_time,
            close_time: msg.close_time,
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data:Result<models::Restaurant, Error> = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(restaurant).values(&new_user).execute(conn)?;
            restaurant.order(restaurant_id.desc()).first(conn)
        });
        match data {
            Ok(mut x) => {
                use self::schema::menu::dsl::*;
                let mid = x.restaurant_id.clone();
                let new_menu = models::NewMenu {
                    restaurant_id: mid.clone(),
                };
                let tmenu:Result<models::Menu, Error> = conn.transaction::<_, Error, _>(|| {
                    diesel::insert_into(menu).values(&new_menu).execute(conn)?;
                    menu.order(menu_id.desc()).first(conn)
                });
                match tmenu {
                    Ok(y) => {
                        let nupdate = models::RestaurantUpdate {
                            restaurant_id: mid.clone(),
                            name: None,
                            phone: None,
                            email: None,
                            enable: None,
                            chain_id: None,
                            menu_id: Some(y.menu_id),
                            open_time: None,
                            close_time: None,
                        };
                        if let Err(x) = diesel::update(restaurant.find(mid)).set(&nupdate).execute(conn) {
                            return Err(error::ErrorInternalServerError(x))
                        };
                        x.menu_id = y.menu_id;
                    },
                    Err(y) => ()
                };
                Ok(x)
            },
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}


impl Message for restaurants::RestaurantPutParams {
    type Result = Result<models::Restaurant, Error>;
}
impl Handler<restaurants::RestaurantPutParams> for DbExecutor {
    type Result = Result<models::Restaurant, Error>;

    fn handle(&mut self, msg: restaurants::RestaurantPutParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl::*;
        println!("{:?}", msg);
        let mid = msg.restaurant_id.clone();
        let new_user = models::RestaurantUpdate {
            restaurant_id: msg.restaurant_id,
            name: msg.name,
            phone: msg.phone,
            email: msg.email,
            enable: msg.enable,
            chain_id: msg.chain_id,
            menu_id: msg.menu_id,
            open_time: msg.open_time,
            close_time: msg.close_time,
        };
        let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::update(restaurant.find(mid)).set(&new_user).execute(conn)?;
            restaurant.find(mid).first(conn)
        });
        match data {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }
    }
}

impl Message for restaurants::RestaurantDeleteParams {
    type Result = Result<(), Error>;
}
impl Handler<restaurants::RestaurantDeleteParams> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: restaurants::RestaurantDeleteParams, _: &mut Self::Context) -> Self::Result {
        use self::schema::restaurant::dsl::*;
        println!("{:?}", msg);
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