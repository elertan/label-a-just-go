use rocket::request::FromParam;
use uuid::{Uuid, parser};

pub struct RocketUuid(Uuid);

impl RocketUuid {
    pub fn to_string(&self) -> String {
        format!("{}", self.0)
    }
    pub fn get_uuid(&self) -> Uuid { self.0 }
}

impl FromParam<'_> for RocketUuid {
    type Error = parser::ParseError;

    fn from_param(param: &'_ rocket::http::RawStr) -> Result<Self, Self::Error> {
        match Uuid::parse_str(param) {
            Ok(val) => Ok(RocketUuid(val)),
            Err(e) => Err(e)
        }
    }
}