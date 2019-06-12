use rocket_contrib::json::JsonValue;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub enum ApiErrorCode {
    NotFound,
    NotImplemented,
    Generic,
    ParserError,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub err_code: ApiErrorCode,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiEmptyData;

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

    pub fn to_json(&self) -> JsonValue {
        json!(self)
    }
}

impl ApiResult<ApiEmptyData> {
    pub fn err(err: ApiError) -> ApiResult<ApiEmptyData> {
        ApiResult {
            data: None,
            err: Some(err),
        }
    }
}
