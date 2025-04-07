/// All routes related to authentication.
mod auth;
/// All routes related to user data.
mod user;
/// All routes related to resources in a workspace.
mod workspace;

use axum::Router;

use crate::prelude::*;

/// Sets up the routes for the API
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.with_state(state.clone())
		.merge(auth::setup_routes(state).await)
		.merge(user::setup_routes(state).await)
		.merge(workspace::setup_routes(state).await)
}
