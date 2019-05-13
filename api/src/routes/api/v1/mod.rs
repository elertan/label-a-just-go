use rocket::data::Data;
use rocket::State;
use rocket_contrib::json::JsonValue;

use crate::extensions::uuid::RocketUuid;
use crate::models::api_result::{ApiError, ApiErrorCode, ApiResult};
use crate::models::user::UserAccount;
use crate::RocketState;
use crate::schema::*;
use crate::diesel::prelude::*;


#[get("/api/v1/registration-details/<token>")]
pub fn registration_details(rocket_state: State<RocketState>, token: RocketUuid) -> JsonValue {
    let conn = rocket_state.conn.lock().expect("Unable to acquire lock on db conn");

    let results = user_account
        .filter(user_account::first_name.eq("Dennis"))
        .limit(1)
        .load::<UserAccount>(&conn)
        .expect("Error loading user");

    ApiResult::success(UserAccount {
        uuid: token.get_uuid(),
        firstname: "Dennis".to_string(),
        surname: "Kievits".to_string(),
    }).to_json()
}

#[post("/api/v1/extract-face", data = "<data>")]
pub fn extract_face(data: Data) -> JsonValue {
    let filepath = format!("./temp/{}.dat", uuid::Uuid::new_v4());
    let result = data.stream_to_file(&filepath).map(|n| n.to_string());
    println!("Bytes received extract_face: {}", result.expect("Uploaded content failed"));

    let extracted_face = crate::utils::facial_recognition::extract_face(filepath.as_str());

    std::fs::remove_file(&filepath).expect("Could not remove temp image file");

    match extracted_face {
        Ok(data) => {
            return ApiResult::success(data).to_json();
        }
        Err(msg) => {
            return ApiResult::err(ApiError {
                err_code: ApiErrorCode::Generic,
                message: "Face extraction script failed".to_string(),
            }).to_json();
        }
    }
}