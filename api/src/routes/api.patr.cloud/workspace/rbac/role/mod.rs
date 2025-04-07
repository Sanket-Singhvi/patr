use axum::Router;

use crate::prelude::*;

/// Create a new role in a workspace. This will create a new role with the
/// provided name, description, and permissions. The permissions will
/// determine what a user with the mentioned role can do in the workspace.
mod create_new_role;
/// Deletes a role from the workspace and revokes the cached permissions. This
/// will delete all the permissions associated with the role. Any user that has
/// the role will have it removed, if the `remove_users` query parameter is set
/// to true. Otherwise, an error will be thrown.
mod delete_role;
/// Get all the details of a role in a workspace. This will return the name,
/// description, and permissions of the role.
mod get_role_info;
/// List all roles in the workspace. This will return all the roles that are
/// available in the workspace, not just the roles of the user. To get the roles
/// of the user, use the [`get_current_permissions`][1] route.
///
/// [1]: super::super::permission::get_current_permissions
mod list_all_roles;
/// List all users for a role in the workspace. This will return all the users
/// that have the role in the workspace.
mod list_users_for_role;
/// Update a role in a workspace. This will update the name, description, and
/// permissions of the role. If the name or permissions are not provided, they
/// will not be updated.
mod update_role;

use self::{
	create_new_role::*,
	delete_role::*,
	get_role_info::*,
	list_all_roles::*,
	list_users_for_role::*,
	update_role::*,
};

/// The handler to setup all routes related to roles. This includes listing all
/// roles, creating a new role, updating a role, and deleting a role.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(create_new_role, state)
		.mount_auth_endpoint(delete_role, state)
		.mount_auth_endpoint(get_role_info, state)
		.mount_auth_endpoint(list_all_roles, state)
		.mount_auth_endpoint(list_users_for_role, state)
		.mount_auth_endpoint(update_role, state)
		.with_state(state.clone())
}
