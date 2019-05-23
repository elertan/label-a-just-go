use crate::utils::{ApiError, ApiErrorCode, ApiResult};
use actix_web::web::Json;

use std::fs;
use std::io::Write;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_service::ServiceExt;
use actix_web::{error, web, Error, HttpResponse};
use futures::future::{err, Either};
use futures::{Future, Stream};

pub fn route_not_found() -> Json<ApiResult<String>> {
    Json(ApiResult::err(ApiError::from_err_code(
        ApiErrorCode::RouteNotFound,
    )))
}

pub fn event_invitation(uuid_string: web::Path<(String)>) -> Json<ApiResult<String>> {
    let parse_result = uuid::Uuid::parse_str(uuid_string.as_str());
    if parse_result.is_err() {
        return Json(ApiResult::err(ApiError::from_err_code(
            ApiErrorCode::InvalidUuid,
        )));
    }

    Json(ApiResult::success(format!("Hello, {}!", uuid_string)))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractFaceResult {
    pub start_x: u32,
    pub end_x: u32,
    pub start_y: u32,
    pub end_y: u32,
}

#[derive(Serialize, Deserialize, Debug, Display)]
pub enum ExtractFaceError {
    #[display(fmt = "No face was found in the given photo")]
    NoFaceFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveFileAndExtractFaceResult {
    pub filename: String,
    pub result: Result<ExtractFaceResult, ExtractFaceError>,
}

pub fn save_file_and_extract_face(
    field: Field,
) -> impl Future<Item = SaveFileAndExtractFaceResult, Error = Error> {
    let file_path_string = format!("{}.png", uuid::Uuid::new_v4());
    let file = match fs::File::create(&file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };

    let content_disposition = field
        .content_disposition()
        .expect("Content disposition not set");

    let filename: String = content_disposition
        .get_filename()
        .expect("Filename not set")
        .to_string();

    Either::B(
        field
            .fold((file, 0u64), move |(mut file, mut read_bytes), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(&bytes).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;

                    read_bytes += bytes.len() as u64;
                    Ok((file, read_bytes))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(move |(_, _)| {
                fs::remove_file(&file_path_string).expect("File could not be removed.");

                SaveFileAndExtractFaceResult {
                    filename,
                    result: Ok(ExtractFaceResult {
                        start_x: 0,
                        end_x: 0,
                        start_y: 0,
                        end_y: 0,
                    }),
                }
            })
            .map_err(|e| {
                println!("Saving file failed: {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

//https://github.com/actix/examples/blob/master/multipart/src/main.rs
pub fn extract_faces(multipart: Multipart) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file_and_extract_face(field).into_stream())
        .flatten()
        .collect()
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
        .map(|data| HttpResponse::Ok().json(ApiResult::success(data)))

    // I want ->
    //    Json(
    //        ApiResult::success(
    //            multipart_data_results.fold() <- Would be a vec of the data in the last map
    //        )
    //    )
}
