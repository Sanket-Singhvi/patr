use axum::Router;

use crate::prelude::*;

/// List all users in the given workspace, along with their roles.
mod list_users_in_workspace;
/// Remove a user from a workspace. This will remove the user from the
/// workspace, and set the revocation timestamp in Redis.
mod remove_user_from_workspace;
/// Update a user's roles in a workspace. This requires the user who is sending
/// the request to have the permission to update roles in the workspace.
mod update_user_roles_in_workspace;

use self::{
	list_users_in_workspace::*,
	remove_user_from_workspace::*,
	update_user_roles_in_workspace::*,
};

/// The handler to setup all routes related to the association of users with
/// roles. This includes listing all users with a role, adding a user to a role,
/// and removing a user from a role.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(list_users_in_workspace, state)
		.mount_auth_endpoint(remove_user_from_workspace, state)
		.mount_auth_endpoint(update_user_roles_in_workspace, state)
}
