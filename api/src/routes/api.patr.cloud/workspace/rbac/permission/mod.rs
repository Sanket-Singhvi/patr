use axum::Router;

use crate::prelude::*;

/// Get the permissions of the current request. This will return the permissions
/// of the currently authenticated user in the workspace.
mod get_current_permissions;
/// List all permissions in the database. This will return all permissions that
/// are available to a user. This will not return the permissions of the user,
/// but all permissions that are available in the database. This is useful for
/// the user to know what permissions are available to them.
mod list_all_permissions;
/// List all resource types in the database. This will return all resource types
/// that are available to a user. This will not return the resource types of the
/// user, but all resource types that are available in the database. This is
/// useful for the user to know what resource types are available to them.
mod list_all_resource_types;

use self::{get_current_permissions::*, list_all_permissions::*, list_all_resource_types::*};

/// The handler to setup all routes related to permissions and resource types.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(get_current_permissions, state)
		.mount_auth_endpoint(list_all_permissions, state)
		.mount_auth_endpoint(list_all_resource_types, state)
		.with_state(state.clone())
}
