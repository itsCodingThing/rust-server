use axum::{
    extract::Multipart,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use crate::utils::response::Response;

pub fn routes() -> Router {
    let api = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload));

    return Router::new().nest("/api", api);
}

async fn index() -> (StatusCode, Json<Response<String>>) {
    let response = Response::<String> {
        msg: "massage".to_string(),
        status_code: StatusCode::OK.as_u16(),
        data: "this is the data".to_string(),
    };

    return response.create_json();
}

async fn upload(mut form: Multipart) -> (StatusCode, Json<Response<String>>) {
    let response = Response::<String> {
        msg: "massage".to_string(),
        status_code: StatusCode::OK.as_u16(),
        data: "this is the data".to_string(),
    };

    while let Some(field) = form.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );
    }


    return response.create_json();
}
