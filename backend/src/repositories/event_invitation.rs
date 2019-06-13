use crate::db::DbExecutor;
use crate::models::EventInvitation;
use actix::{Handler, Message};

#[derive(Debug)]
pub struct GetEventInvitationByToken {
    token: uuid::Uuid,
}

pub enum GetEventInvitationByTokenError {
    NotFound
}

type ResultType = Result<EventInvitation, GetEventInvitationByTokenError>;

impl Message for GetEventInvitationByToken {
    type Result = ResultType;
}

impl Handler<GetEventInvitationByToken> for DbExecutor {
    type Result = ResultType;

    fn handle(&mut self, data: GetEventInvitationByToken, _: &mut Self::Context) -> Self::Result {
        use crate::schema::event_invitation::dsl::*;
        use diesel::prelude::*;

        let conn: &PgConnection = &self.0.get().unwrap();

        let result: Vec<EventInvitation> = event_invitation
            .filter(token.eq(data.token))
            .limit(1)
            .load(conn)
            .expect("Unable to execute query for event invitation");

        match result.first() {
            Some(&event_invite) => Ok(event_invite),
            None => Err(GetEventInvitationByTokenError::NotFound)
        }
    }
}