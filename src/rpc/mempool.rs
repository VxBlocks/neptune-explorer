use crate::http_util::rpc_err;
use crate::http_util::rpc_method_err;
use crate::model::app_state::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Json;
use axum::response::Response;
use neptune_cash::rpc_server::MempoolTransactionInfo;
use neptune_cash::rpc_server::ProofOfWorkPuzzle;
use std::sync::Arc;
use tarpc::context;

#[axum::debug_handler]
pub async fn mempool_overview(
    Path((start_index, number)): Path<(usize, usize)>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MempoolTransactionInfo>>, Response> {
    let s = state.load();
    let mempool_transaction_infos = s
        .rpc_client
        .mempool_overview(context::current(), s.token(), start_index, number)
        .await
        .map_err(rpc_err)?
        .map_err(rpc_method_err)?;

    Ok(Json(mempool_transaction_infos))
}

pub async fn pow_puzzle_internal_key(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Option<ProofOfWorkPuzzle>>, Response> {
    let s = state.load();
    let mempool_transaction_infos = s
        .rpc_client
        .pow_puzzle_internal_key(context::current(), s.token())
        .await
        .map_err(rpc_err)?
        .map_err(rpc_method_err)?;

    Ok(Json(mempool_transaction_infos))
}