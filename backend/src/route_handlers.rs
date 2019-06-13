use crate::utils::{ApiError, ApiErrorCode, ApiResult};
use actix_web::web::Json;

use std::fs;
use std::io::Write;

use crate::repositories::event_invitation::GetEventInvitationByToken;
use crate::AppState;
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

pub fn event_invitation(
    state: web::Data<AppState>, uuid_string: web::Path<String>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    let parse_result = uuid::Uuid::parse_str(uuid_string.as_str());
    if parse_result.is_err() {
        return futures::future::ok(
            HttpResponse::Ok().json(
                ApiResult::err(
                    ApiError::from_err_code(
                        ApiErrorCode::InvalidUuid,
                    )
                )
            )
        );
    }


    let msg = GetEventInvitationByToken {
        token: parse_result.unwrap()
    };
    state
        .db
        .send(msg)
        .from_err()
        .and_then(|res| match res {
            Ok(val) => Ok(HttpResponse::Ok().json(val)),
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        })

//    let msg = GetAll;
//    state.db.send(msg).from_err().and_then(|res| match res {
//        Ok(val) => Ok(HttpResponse::Ok().json(val)),
//        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
//    })
    //    Json(ApiResult::success(format!("Hello, {}!", uuid_string)))
}

//#[derive(Serialize, Deserialize, Debug)]
//pub struct ExtractFaceResult {
//    pub start_x: u32,
//    pub end_x: u32,
//    pub start_y: u32,
//    pub end_y: u32,
//}

#[derive(Serialize, Deserialize, Debug, Display)]
pub enum ExtractFaceError {
    #[display(fmt = "No face was found in the given photo")]
    NoFaceFound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtractFaceFromFieldResult {
    pub filename: String,
    pub data: Option<crate::utils::facial_recognition::ExtractFaceResult>,
    pub err: Option<ExtractFaceError>,
}

/// Extracts faces from a field type and stores them into a ExtractFaceFromFieldResult
pub fn extract_faces_from_field(
    field: Field,
) -> impl Future<Item=ExtractFaceFromFieldResult, Error=Error> {
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
                let path = format!("../{}", &file_path_string);
                let result = crate::utils::facial_recognition::extract_face(path.as_str());

                fs::remove_file(&file_path_string).expect("File could not be removed.");

                match result {
                    Ok(data) => ExtractFaceFromFieldResult {
                        filename,
                        data: Some(data),
                        err: None,
                    },
                    Err(_) => ExtractFaceFromFieldResult {
                        filename,
                        data: None,
                        err: Some(ExtractFaceError::NoFaceFound),
                    },
                }
            })
            .map_err(|e| {
                println!("Saving file failed: {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

//https://github.com/actix/examples/blob/master/multipart/src/main.rs
/// Extracts the faces for a multipart request type
pub fn extract_faces(multipart: Multipart) -> impl Future<Item=HttpResponse, Error=Error> {
    multipart
        // If any field of the multipart fails, throw an error
        .map_err(error::ErrorInternalServerError)
        // For every field of the multipart data do the face extraction process
        .map(|field| extract_faces_from_field(field).into_stream())
        // Flatten the array of streams into a single vector
        .flatten()
        // Collect the output
        .collect()
        // If the face extraction failed for one of these processes handle the error
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
        // Map the output into a http response of an apiresult
        .map(|data| HttpResponse::Ok().json(ApiResult::success(data)))
}
