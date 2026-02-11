use axum::Json;
use reqwest::StatusCode;
use serde::Serialize;

pub type Response<T> = (StatusCode, Json<APIResponse<T>>);

#[derive(Debug, Serialize, Clone)]
pub struct APIResponse<T: Serialize + Clone> {
    status: String,
    message: Option<String>,
    data: Option<T>,
}

impl<T: Serialize + Clone> APIResponse<T> {
    fn new(status: StatusCode, response: Option<T>) -> Self {
        Self {
            status: status.to_string(),
            data: response,
            message: None,
        }
    }

    fn err(status: StatusCode, err: String) -> Self {
        Self {
            status: status.to_string(),
            data: None,
            message: Some(err),
        }
    }
}

pub fn api_response<T: Serialize + Clone>(
    status: StatusCode,
    response: T,
) -> (StatusCode, Json<APIResponse<T>>) {
    (status, Json(APIResponse::new(status, Some(response))))
}

pub fn err_response<T: Serialize + Clone>(
    status: StatusCode,
    err: String,
) -> (StatusCode, Json<APIResponse<T>>) {
    (status, Json(APIResponse::err(status, err)))
}
