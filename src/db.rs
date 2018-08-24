//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use models;
use schema;

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub DBPool);

/// State with DbExecutor address
pub struct AppState {
    pub db: Addr<DbExecutor>,
}

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
#[derive(Debug)]
pub struct CreateMember {
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub gender: i8,
}


impl Message for CreateMember {
//  type Result = Result<models::Member, Error>;
    type Result = Result<(), Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateMember> for DbExecutor {
//  type Result = Result<models::Member, Error>;
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: CreateMember, _: &mut Self::Context) -> Self::Result {
        /* r2d2 fail so comment*/
        /*use self::schema::member::dsl::*;
        println!("{:?}", msg);
        let mut new_user = models::NewMember {
            email: msg.email,
            name: msg.name,
            password: msg.password,
            gender: msg.gender,
            phone_number: "".to_string(),
        };
        if let Some(x) = msg.phone_number {
            new_user.phone_number = x.clone();
        };
        let conn = self.0.get();
        let conn: &MysqlConnection = &conn.unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<models::Member, Error, _>(|| {
            diesel::insert_into(member).values(&new_user).execute(conn)?;
            member.order(member_id.desc()).first(conn)
        }).unwrap();*/
        Ok(())
    }
}

