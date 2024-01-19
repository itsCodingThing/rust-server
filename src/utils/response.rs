use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<D> {
    pub msg: String,
    pub status_code: u16,
    pub data: D,
}

impl<D> Response<D> {
    pub fn create_json(self) -> (StatusCode, Json<Response<D>>) {
        return (StatusCode::from_u16(self.status_code).unwrap(), Json(self));
    }
}
