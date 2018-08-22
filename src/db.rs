//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use models;
use schema;

pub type DBPool = Pool<ConnectionManager<MysqlConnection>>;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub DBPool);

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
#[derive(Debug)]
pub struct CreateMember {
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
    pub password: String,
    pub member_level: i8,
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
        /* r2d2 fail so comment
        use self::schema::member::dsl::*;
        println!("{:?}", msg);
        let mut new_user = models::NewMember {
            email: msg.email,
            name: msg.name,
            password: msg.password,
            member_level: msg.member_level,
            phone_number: "".to_string(),
        };
        if let Some(x) = msg.phone_number {
            new_user.phone_number = x.clone();
        };
        let conn: MysqlConnection = MysqlConnection::establish("mysql://eat:eateat@localhost/eat").unwrap();
        //let conn: &MysqlConnection = &self.0.get().unwrap();
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(member).values(&new_user).execute(&conn)?;
            member.order(member_id.desc()).first(&conn)
        });
        match data  {
            Ok(x) => Ok(x),
            Err(x) => Err(error::ErrorInternalServerError(x))
        }*/
        Ok(())
    }
}

