use crate::models::user::User;
use rocket_contrib::json::JsonValue;
use crate::extensions::uuid::RocketUuid;
use crate::models::api_result::ApiResult;

#[get("/api/v1/registration-details/<token>")]
pub fn registration_details(token: RocketUuid) -> JsonValue {
    ApiResult::success(User {
        uuid: token.to_string(),
        firstname: "Dennis".to_string(),
        lastname: "Kievits".to_string(),
    }).to_json()
}