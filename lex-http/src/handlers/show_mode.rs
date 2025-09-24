use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;

use lex_service::models::mode::{Mode, ModeId};
use lex_service::ports::LexService;

use crate::responses::api_error::ApiError;
use crate::responses::api_success::ApiSuccess;
use crate::server::AppState;

/// Shows the search fields that are available in a [Mode].
///
/// # Responses
///
/// - 200 Ok: the search fields for the requested [Mode].
/// - 404 Not Found: if the [Mode] doesn't exist.
pub async fn show_mode<LS: LexService>(
    State(state): State<AppState<LS>>,
    Path(mode): Path<String>,
) -> Result<ApiSuccess<ShowModeResponseData>, ApiError> {
    state
        .lex_service
        .get_fieldmappings(&mode)
        .await
        .map_err(ApiError::from)
        .map(|ref modes| ApiSuccess::new(StatusCode::OK, modes.into()))
}
#[derive(Clone, Debug, serde::Serialize, PartialEq, Eq)]
pub struct ShowModeResponseData(HashMap<ModeId, Vec<ModeId>>);

impl From<&Vec<Mode>> for ShowModeResponseData {
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
