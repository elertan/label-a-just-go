use crate::models::user::User;
use crate::extensions::uuid::RocketUuid;
use crate::models::api_result::{ApiResult, ApiError, ApiErrorCode};
use rocket::data::Data;
use rocket_contrib::json::JsonValue;

#[get("/api/v1/registration-details/<token>")]
pub fn registration_details(token: RocketUuid) -> JsonValue {
    ApiResult::success(User {
        uuid: token.to_string(),
        firstname: "Dennis".to_string(),
        lastname: "Kievits".to_string(),
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