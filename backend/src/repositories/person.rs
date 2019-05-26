use crate::db::DbExecutor;
use crate::models::Person;
use actix::{Handler, Message};

use std::io::Error;

pub struct GetAll;

impl Message for GetAll {
    type Result = Result<Vec<Person>, Error>;
}

impl Handler<GetAll> for DbExecutor {
    type Result = Result<Vec<Person>, Error>;

    fn handle(&mut self, _: GetAll, _: &mut Self::Context) -> Self::Result {
        use crate::schema::person::dsl::*;
        use diesel::prelude::*;

        let conn: &PgConnection = &self.0.get().unwrap();
        let result = person
            .filter(id.eq(2))
            .limit(5)
            .load::<Person>(conn)
            .expect("bs");

        Ok(result)
    }
}
