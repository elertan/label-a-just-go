//use diesel_derive_enum::DbEnum;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "person"]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "event"]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
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
    pub id: i32,
    pub person_id: i32,
    pub event_id: i32,
    pub token: uuid::Uuid,
    pub status: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable)]
#[table_name = "photo"]
pub struct Photo {
    pub id: i32,
    pub person_id: i32,
    pub filename: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}
