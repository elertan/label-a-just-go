//use diesel_derive_enum::DbEnum;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "person"]
pub struct Person {
    pub id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "event"]
pub struct Event {
    pub id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub name: String,
}

//#[derive(Debug, DbEnum, Clone, Serialize, Deserialize, PartialEq)]
//#[sql_type = "diesel::sql_types::Int4"]
//pub enum EventInvitationStatus {
//    Pending = 0,
//    Accepted = 1,
//    Declined = 2,
//    Revoked = 3,
//}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "event_invitation"]
pub struct EventInvitation {
    pub id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub event_id: u32,
    pub person_id: u32,
    pub status: u32,
    pub token: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "photo"]
pub struct Photo {
    pub id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub person_id: u32,
}
