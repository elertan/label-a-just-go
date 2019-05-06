use crate::models::user::User;
use rocket_contrib::json::{JsonValue};
use crate::extensions::uuid::RocketUuid;
use crate::models::api_result::{ApiResult, ApiError, ApiErrorCode};
use rocket::data::Data;
use std::process::{Command, Stdio};

#[get("/api/v1/registration-details/<token>")]
pub fn registration_details(token: RocketUuid) -> JsonValue {
    ApiResult::success(User {
        uuid: token.to_string(),
        firstname: "Dennis".to_string(),
        lastname: "Kievits".to_string(),
    }).to_json()
}

#[derive(Serialize, Deserialize)]
struct ExtractFacePythonResult {
    pub l: u32,
    pub r: u32,
    pub t: u32,
    pub b: u32,
}

#[post("/api/v1/extract-face/<token>", data = "<data>")]
pub fn extract_face(token: RocketUuid, data: Data) -> JsonValue {
    let filename = format!("{}.temp", uuid::Uuid::new_v4());
    let result = data.stream_to_file(&filename).map(|n| n.to_string());
    println!("data: {}", result.expect("Uploaded content failed"));

//    python extract_face.py --detector face_detection_model --embedding-model openface_nn4.small2.v1.t7 --image test.temp
    let output = Command::new("python")
//        .current_dir(std::fs::canonicalize("./python").unwrap())
        .stdout(Stdio::piped())
        .args(&[
            "extract_face.py",
            "--detector",
            "face_detection_model",
            "--embedding-model",
            "openface_nn4.small2.v1.t7",
            "--image",
            filename.as_str()
        ])
        .output()
        .expect("failed to execute process");

    std::fs::remove_file(&filename).expect("Could not remove file");

    if !output.status.success() {
        return ApiResult::err(ApiError {
            err_code: ApiErrorCode::ParserError,
            message: "Face extraction script failed".to_string(),
        }).to_json();
    }
    let output_string_raw = String::from_utf8_lossy(&output.stdout).to_string();
    let output_string = output_string_raw.trim();
    let output_data: ExtractFacePythonResult = serde_json::from_str(output_string).unwrap();

    ApiResult::success(output_data).to_json()
}