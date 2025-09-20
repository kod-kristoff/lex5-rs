use axum::extract::State;

use lex_service::ports::LexService;

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
