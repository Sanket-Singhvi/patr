use axum::Router;

use crate::prelude::*;

// mod container_registry;
#[allow(
	unreachable_code,
	unused_variables,
	clippy::missing_docs_in_private_items,
	missing_docs
)]
mod database;
/// All routes for a workspace's deployments. Deployments are used to deploy
/// containers to a runner. They can be created, updated, and deleted.
/// Deployments can be associated with a specific runner and can be scaled up or
/// down.
mod deployment;
#[allow(
	unreachable_code,
	unused_variables,
	clippy::missing_docs_in_private_items,
	missing_docs
)]
mod domain;
/// All routes for a workspace's managed URLs. Managed URLs are used to handle
/// incoming traffic. They can be used to route traffic to a specific deployment
/// or to a static site. Managed URLs can be configured to handle traffic in
/// different ways, such as redirecting to another URL or serving a static site.
mod managed_url;
/// All routes for a workspace's RBAC. RBAC is used to manage permissions and
/// roles for a workspace. Permissions are used to control access to resources
/// and actions. Roles are used to group permissions together and assign them to
/// users.
///
/// So in essence:
/// - Users belong to a workspace through a role.
/// - A user can have multiple roles in a workspace.
/// - A workspace can possibly have a default role.
/// - Roles are collections of permissions on resources.
///     - Either the role can have specific permissions on a resource.
///     - Or the role can have specific permissions on all resources.
///     - Or the role can have specific permissions on all resources except
///       some.
/// - Users need to have specific permission on a resource to perform that
///   action.
/// - Resources _need_ to be a part of a workspace.
/// - A workspace has a super admin that can do anything on the workspace.
mod rbac;
/// All routes for a workspace's runners. Runners are used to deploy
/// applications to a workspace. They are associated with a user and a
/// workspace.
mod runner;
#[allow(
	unreachable_code,
	unused_variables,
	clippy::missing_docs_in_private_items,
	missing_docs
)]
mod secret;
#[allow(
	unreachable_code,
	unused_variables,
	clippy::missing_docs_in_private_items,
	missing_docs
)]
mod static_site;
/// All routes for a workspace's volumes. Volumes can be created, grown and
/// deleted. They are used to store data that persists between deployments.
/// Volumes are provided a name for easy identification. Eventually, backups for
/// volumes will be associated with the volume name.
mod volume;

/// The handler to create a new workspace. The workspace name must be unique.
mod create_workspace;
/// The handler to delete a workspace. This will delete all associated data
/// with the workspace, including the database, container registry, and any
/// other resources. This is a destructive operation and cannot be undone.
/// The workspace must be empty before it can be deleted.
mod delete_workspace;
/// The handler to get the information of a workspace. This includes the
/// workspace's name, the user who created it, and the date it was created.
mod get_workspace_info;
/// The handler to check if a workspace name is available. This is used when
/// creating a new workspace to ensure that the name is unique.
mod is_name_available;
/// The handler to update the information of a workspace. At the moment, only
/// the name can be updated. However, this will be expanded in the future. At
/// least one parameter must be provided for the update.
mod update_workspace_info;

use self::{
	create_workspace::*,
	delete_workspace::*,
	get_workspace_info::*,
	is_name_available::*,
	update_workspace_info::*,
};

#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		// .merge(container_registry::setup_routes(state).await)
		.merge(domain::setup_routes(state).await)
		.merge(database::setup_routes(state).await)
		.merge(deployment::setup_routes(state).await)
		.merge(managed_url::setup_routes(state).await)
		.merge(rbac::setup_routes(state).await)
		.merge(runner::setup_routes(state).await)
		.merge(secret::setup_routes(state).await)
		.merge(static_site::setup_routes(state).await)
		.merge(volume::setup_routes(state).await)
		.mount_auth_endpoint(create_workspace, state)
		.mount_auth_endpoint(delete_workspace, state)
		.mount_auth_endpoint(get_workspace_info, state)
		.mount_auth_endpoint(is_name_available, state)
		.mount_auth_endpoint(update_workspace_info, state)
}
