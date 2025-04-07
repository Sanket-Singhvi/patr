use axum::Router;

use crate::prelude::*;

/// All routes related to permissions and resource types.
mod permission;
/// All routes related to roles. This includes listing all roles, creating a new
/// role, updating a role, and deleting a role.
mod role;
/// All routes related to the association of users with roles. This includes
/// listing all users with a role, adding a user to a role, and removing a user
/// from a role.
mod user;

/// The handler to setup all routes for the RBAC workspace. This will setup all
/// the routes for the RBAC workspace, including the routes for permissions,
/// roles, and users.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.merge(permission::setup_routes(state).await)
		.merge(role::setup_routes(state).await)
		.merge(user::setup_routes(state).await)
}
