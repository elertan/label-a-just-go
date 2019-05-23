use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Display, Clone)]
pub enum ApiErrorCode {
    /// An invalid route, which is not registered within the application
    #[display(fmt = "The requested route was not found on the server")]
    RouteNotFound,
    #[display(fmt = "The given uuid is not valid")]
    InvalidUuid,
    #[display(fmt = "The required file was not uploaded")]
    FileNotUploaded,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub err_code: ApiErrorCode,
    pub message: String,
}

impl ApiError {
    pub fn from_err_code(code: ApiErrorCode) -> Self {
        ApiError {
            message: format!("{}", &code),
            err_code: code,
        }
    }

    pub fn from_err_code_with_msg(code: ApiErrorCode, message: String) -> Self {
        ApiError {
            err_code: code,
            message,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResult<TData>
where
    TData: Serialize,
{
    pub data: Option<TData>,
    pub err: Option<ApiError>,
}

impl<TData> ApiResult<TData>
where
    TData: Serialize,
{
    pub fn success(data: TData) -> ApiResult<TData> {
        ApiResult {
            data: Some(data),
            err: None,
        }
    }

    pub fn err(err: ApiError) -> ApiResult<TData> {
        ApiResult {
            data: None,
            err: Some(err),
        }
    }
}
