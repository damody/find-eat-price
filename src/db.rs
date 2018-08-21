//! Db executor actor
use actix::prelude::*;
use actix_web::*;
use diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid;


use models;
use schema;

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<MysqlConnection>>);

/// This is only message that this actor can handle, but it is easy to extend
/// number of messages.
pub struct CreateMember {
    pub email: String,
    pub name: String,
    pub phone_number: String,
    pub password: String,
    pub member_level: i8,
}

impl Message for CreateMember {
    type Result = Result<models::Member, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateMember> for DbExecutor {
    type Result = Result<models::Member, Error>;

    fn handle(&mut self, msg: CreateMember, _: &mut Self::Context) -> Self::Result {
        use self::schema::member::dsl::*;
        
        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_user = models::NewMember {
            email: &msg.email,
            name: &msg.name,
            phone_number: &msg.phone_number,
            password: &msg.password,
            member_level: msg.member_level,
        };

        let conn: &MysqlConnection = &self.0.get().unwrap();

        /*diesel::insert_into(member)
            .values(&new_user)
            .get_result(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting person"))?;*/
        use diesel::result::Error;
        let data = conn.transaction::<_, Error, _>(|| {
            diesel::insert_into(member).values(&new_user).execute(conn)?;
            member.order(member_id.desc()).first(conn)
        }).unwrap();
        /*
        let mut items = member
            .filter(id.eq(&uuid))
            .load::<models::Member>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading person"))?;
        */
        Ok(data)
    }
}
