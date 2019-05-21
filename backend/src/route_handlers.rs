use crate::utils::{ApiError, ApiErrorCode, ApiResult};
use actix_web::web::Json;
use actix_web::{web, Error};

pub fn route_not_found() -> Json<ApiResult<String>> {
    Json(
        ApiResult::err(
            ApiError::from_err_code(ApiErrorCode::RouteNotFound)
        )
    )
}

pub fn event_invitation(uuid_string: web::Path<(String)>) -> Json<ApiResult<String>> {
    let parse_result = uuid::Uuid::parse_str(uuid_string.as_str());
    if let Err(_) = parse_result {
        return Json(
            ApiResult::err(
                ApiError::from_err_code(ApiErrorCode::InvalidUuid)
            )
        );
    }

    Json(
        ApiResult::success(
            format!("Hello, {}!", uuid_string)
        )
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractFaceResult {
    pub start_x: u32,
    pub end_x: u32,
    pub start_y: u32,
    pub end_y: u32
}

//https://github.com/actix/examples/blob/master/multipart/src/main.rs
//pub fn extract_face(multipart: Multipart) -> Future<Item = Json<ApiResult<ExtractFaceResult>>, Error = Error> {
//    let result = multipart
//        .map_err(error::ErrorInternalServerError)
//        .map(|field| save_file(field).into_stream())
//        .flatten()
//        .collect()
//        .map(|sizes| HttpResponse::Ok().json(sizes))
//        .map_err(|e| {
//            println!("failed: {}", e);
//            e
//        });
//
////    let content_disposition = field.content_disposition();
////    if content_disposition == None {
////        return Json(
////            ApiResult::err(
////                ApiError::from_err_code(ApiErrorCode::FileNotUploaded)
////            )
////        );
////    }
//    Either::B(
//
//        Json(
//            ApiResult::success(
//                ExtractFaceResult {
//                    start_x: 0,
//                    end_x: 0,
//                    start_y: 0,
//                    end_y: 0
//                }
//            )
//        )
//    )
//}