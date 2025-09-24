use std::collections::HashMap;

use axum::extract::State;
use axum::http::StatusCode;

use lex_service::models::mode::{Mode, ModeId};
use lex_service::ports::LexService;

use crate::responses::api_error::ApiError;
use crate::responses::api_success::ApiSuccess;
use crate::server::AppState;

/// List all [Mode]s.
///
/// # Responses
///
/// - 200 Ok: all [Mode]s.
pub async fn list_modes<LS: LexService>(
    State(state): State<AppState<LS>>,
) -> Result<ApiSuccess<ListAllModesResponseData>, ApiError> {
    state
        .lex_service
        .all_modes()
        .await
        .map_err(ApiError::from)
        .map(|ref modes| ApiSuccess::new(StatusCode::OK, modes.into()))
}

#[derive(Clone, Debug, serde::Serialize, PartialEq, Eq)]
pub struct ListAllModesResponseData(HashMap<ModeId, Vec<ModeId>>);

impl From<&Vec<Mode>> for ListAllModesResponseData {
    fn from(modes: &Vec<Mode>) -> Self {
        let mut jsonmodes = HashMap::new();
        for mode in modes {
            jsonmodes.insert(
                mode.id().clone(),
                mode.groups().cloned().unwrap_or_default(),
            );
        }
        Self(jsonmodes)
    }
}
