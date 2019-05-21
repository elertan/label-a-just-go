use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ApiErrorCode {
    RouteNotFound,
    InvalidUuid,
    FileNotUploaded
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub err_code: ApiErrorCode,
    pub message: String,
}

const API_ERROR_CODE_MESSAGE_MAPPING: &[(ApiErrorCode, &'static str)] = &[
    (ApiErrorCode::RouteNotFound, "The requested route was not found on the server."),
    (ApiErrorCode::InvalidUuid, "The provided uuid is invalid."),
    (ApiErrorCode::FileNotUploaded, "The requested file was not uploaded.")
];

impl ApiError {
    pub fn from_err_code(code: ApiErrorCode) -> Self {
        let message = API_ERROR_CODE_MESSAGE_MAPPING
            .iter()
            .find(|&x| x.0 == code)
            .expect("Message not found for error code");

        ApiError {
            err_code: code,
            message: message.1.to_string()
        }
    }

    pub fn from_err_code_with_msg(code: ApiErrorCode, message: String) -> Self {
        ApiError {
            err_code: code,
            message
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResult<TData> where TData: Serialize {
    pub data: Option<TData>,
    pub err: Option<ApiError>,
}

impl<TData> ApiResult<TData> where TData: Serialize {
    pub fn success(data: TData) -> ApiResult<TData> {
        ApiResult {
            data: Some(data),
            err: None
        }
    }

    pub fn err(err: ApiError) -> ApiResult<TData> {
        ApiResult {
            data: None,
            err: Some(err)
        }
    }
}


