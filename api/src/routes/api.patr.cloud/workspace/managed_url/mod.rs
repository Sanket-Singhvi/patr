use axum::Router;

use crate::prelude::*;

/// This will create a new managed URL with the provided subdomain, domain, and
/// path. The URL type can be a proxy to a deployment, a proxy to a static site,
/// a proxy to a URL, or a redirect to a URL. The URL type will determine how
/// the managed URL behaves.
mod create_managed_url;
/// This will delete the managed URL and remove it from the workspace. The
/// managed URL must be owned by the user and not already deleted.
mod delete_managed_url;
/// This will return all managed URLs that the user has access to in the
/// workspace.
mod list_managed_url;
/// Update a managed URL. This will update the managed URL with the new details
/// provided.
mod update_managed_url;
#[allow(
	unreachable_code,
	unused_variables,
	clippy::missing_docs_in_private_items,
	missing_docs
)]
mod verify_configuration;

use self::{
	create_managed_url::*,
	delete_managed_url::*,
	list_managed_url::*,
	update_managed_url::*,
	verify_configuration::*,
};

#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(create_managed_url, state)
		.mount_auth_endpoint(delete_managed_url, state)
		.mount_auth_endpoint(list_managed_url, state)
		.mount_auth_endpoint(update_managed_url, state)
		.mount_auth_endpoint(verify_configuration, state)
}
