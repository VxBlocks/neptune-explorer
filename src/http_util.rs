use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use tarpc::client::RpcError;

// note: http StatusCodes are defined at:
// https://docs.rs/http/1.1.0/http/status/struct.StatusCode.html

pub fn not_found_err() -> Response {
    (StatusCode::NOT_FOUND, "Not Found".to_string()).into_response()
}

pub fn rpc_err(e: RpcError) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
}
