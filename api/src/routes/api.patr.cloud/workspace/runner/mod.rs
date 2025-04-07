use axum::Router;

use crate::prelude::*;

/// This will add a runner to the workspace. The runner will be able to access
/// the workspace and deploy applications.
mod add_runner_to_workspace;
/// This will get the information of a runner in the workspace.
mod get_runner_info;
/// This will list all the runners that the user has access to in the workspace.
mod list_runners_for_workspace;
/// This will remove the runner from the workspace and stop any associated
/// deployments. The runner will no longer be able to access the workspace.
mod remove_runner_from_workspace;
/// For a given runner, this handler streams the changes to the workspace
/// associated with the runner. The runner can use this to reflect the changes
/// in the workspace in real-time.
mod stream_runner_data_for_workspace;

use self::{
	add_runner_to_workspace::*,
	get_runner_info::*,
	list_runners_for_workspace::*,
	remove_runner_from_workspace::*,
	stream_runner_data_for_workspace::*,
};

#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(stream_runner_data_for_workspace, state)
		.mount_auth_endpoint(add_runner_to_workspace, state)
		.mount_auth_endpoint(remove_runner_from_workspace, state)
		.mount_auth_endpoint(list_runners_for_workspace, state)
		.mount_auth_endpoint(get_runner_info, state)
}
