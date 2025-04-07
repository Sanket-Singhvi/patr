use axum::Router;

use crate::prelude::*;

/// Delete a deployment's particular history of deploys, using the image digest.
mod delete_deploy_history;
/// List a deployment's history of deploys. This includes the image digest and
/// the time it was deployed.
mod list_deploy_history;

use self::{delete_deploy_history::*, list_deploy_history::*};

/// The handler to setup all routes for the deployment history stuff. This will
/// setup all the routes for the deployment history stuff, including the routes
/// for listing and deleting the deployment history.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(list_deploy_history, state)
		.mount_auth_endpoint(delete_deploy_history, state)
}
