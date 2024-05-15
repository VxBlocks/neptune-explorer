use crate::html::component::header::HeaderHtml;
use crate::http_util::not_found_html_err;
use crate::http_util::not_found_html_handler;
use crate::model::app_state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Response;
use html_escaper::Escape;
// use html_escaper::Trusted;
use std::sync::Arc;

// #[axum::debug_handler]
pub fn not_found_page(
    State(state): State<Arc<AppState>>,
    error_msg: Option<String>,
) -> Html<String> {
    #[derive(boilerplate::Boilerplate)]
    #[boilerplate(filename = "web/html/page/not_found.html")]
    #[allow(dead_code)]
    pub struct NotFoundHtmlPage {
        header: HeaderHtml,
        error_msg: String,
    }

    let header = HeaderHtml {
        state: state.clone(),
    };

    let not_found_page = NotFoundHtmlPage {
        header,
        error_msg: error_msg.unwrap_or_default(),
    };
    Html(not_found_page.to_string())
}

pub fn not_found_html_response(
    State(state): State<Arc<AppState>>,
    error_msg: Option<String>,
) -> Response {
    not_found_html_err(not_found_page(State(state), error_msg))
}

pub fn not_found_html_fallback(state: Arc<AppState>) -> (StatusCode, Html<String>) {
    not_found_html_handler(not_found_page(State(state), None))
}
