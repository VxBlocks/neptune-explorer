use crate::html::component::header::HeaderHtml;
use crate::html::page::not_found::not_found_html_response;
use crate::http_util::rpc_method_err;
use crate::model::app_state::AppState;
use axum::extract::rejection::PathRejection;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use axum::response::Response;
use html_escaper::Escape;
use html_escaper::Trusted;
use neptune_cash::prelude::tasm_lib::prelude::Digest;
use std::sync::Arc;
use tarpc::context;

#[axum::debug_handler]
pub async fn utxo_page(
    index_maybe: Result<Path<u64>, PathRejection>,
    State(state_rw): State<Arc<AppState>>,
) -> Result<Html<String>, Response> {
    #[derive(boilerplate::Boilerplate)]
    #[boilerplate(filename = "web/html/page/utxo.html")]
    pub struct UtxoHtmlPage<'a> {
        header: HeaderHtml<'a>,
        index: u64,
        digest: Digest,
    }

    let state = &state_rw.load();

    let Path(index) =
        index_maybe.map_err(|e| not_found_html_response(state, Some(e.to_string())))?;

    let digest = match state
        .rpc_client
        .utxo_digest(context::current(), state.token(), index)
        .await
        .map_err(|e| not_found_html_response(state, Some(e.to_string())))?
        .map_err(rpc_method_err)?
    {
        Some(digest) => digest,
        None => {
            return Err(not_found_html_response(
                state,
                Some("The requested UTXO does not exist".to_string()),
            ))
        }
    };

    let header = HeaderHtml { state };

    let utxo_page = UtxoHtmlPage {
        index,
        header,
        digest,
    };
    Ok(Html(utxo_page.to_string()))
}
