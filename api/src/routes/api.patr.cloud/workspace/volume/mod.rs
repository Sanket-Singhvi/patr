use axum::Router;

/// Create a new volume in a workspace. This will create a new volume and
/// return the ID of the volume.
mod create_volume;
/// Delete a volume. This will delete the volume and all the data in the
/// volume. This will also remove the volume from any deployments that are
/// using the volume. This is a destructive operation.
mod delete_volume;
/// Get the information of a volume. This includes the name, size, and the ID of
/// the deployment that is using the volume.
mod get_volume_info;
/// List all the volumes in a workspace.
mod list_volumes;
/// Update volume. Mostly used to update the name and increase the size of the
/// volume.
mod update_volume;

use self::{
	create_volume::*,
	delete_volume::*,
	get_volume_info::*,
	list_volumes::*,
	update_volume::*,
};
use crate::prelude::*;

/// The handler to setup all routes for the volumes. This will setup all the
/// routes for the volumes, including the routes for creating, deleting, getting
/// information, listing, and updating volumes.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.mount_auth_endpoint(create_volume, state)
		.mount_auth_endpoint(delete_volume, state)
		.mount_auth_endpoint(get_volume_info, state)
		.mount_auth_endpoint(list_volumes, state)
		.mount_auth_endpoint(update_volume, state)
}
