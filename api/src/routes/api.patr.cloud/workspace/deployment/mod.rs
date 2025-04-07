use axum::Router;

/// The history of deploys for a deployment. This includes the status of the
/// deploy, and the time it was deployed.
pub mod deploy_history;

/// Create a deployment in the workspace. This will create a new deployment in
/// the workspace, and return the ID of the deployment.
mod create_deployment;
/// Delete a deployment in the workspace. This will delete the deployment from
/// the workspace, and remove all resources associated with the deployment.
mod delete_deployment;
/// Get the deployment info in the workspace. This will return the deployment
/// details for the given deployment ID.
mod get_deployment_info;
/// Route to get the logs of a deployment. This will fetch logs from Loki
/// and return them to the user. The logs can be filtered by time and search
/// query.
mod get_deployment_logs;
/// Route to get the metrics of a deployment. This will fetch metrics from Mimir
/// and return them to the user. The metrics can be filtered by the end time.
mod get_deployment_metric;
/// List all deployment machine types. This is a public endpoint. No
/// authentication is required. This endpoint is used to list all the machine
/// types that are available for deployments.
mod list_all_deployment_machine_types;
/// List all deployments in the workspace. This will return all the deployments
/// in the workspace.
mod list_deployment;
/// Start a deployment in the workspace. This will start the deployment. In case
/// the deployment is already running, it will do nothing.
mod start_deployment;
/// Stop a deployment in the workspace. This will stop the deployment. In case
/// the deployment is already stopped, it will do nothing.
mod stop_deployment;
/// Route to stream the logs of a deployment. This will stream logs from Loki
/// and return them to the user. The logs can be filtered by the start time.
mod stream_deployment_logs;
/// Update deployment details. This endpoint is used to update the deployment
/// details. The deployment details that can be updated are the name, machine
/// type, deploy on push, min horizontal scale, max horizontal scale, ports,
/// environment variables, startup probe, liveness probe, config mounts, and
/// volumes. At least one of the values must be updated.
mod update_deployment;

use self::{
	create_deployment::*,
	delete_deployment::*,
	get_deployment_info::*,
	get_deployment_logs::*,
	get_deployment_metric::*,
	list_all_deployment_machine_types::*,
	list_deployment::*,
	start_deployment::*,
	stop_deployment::*,
	stream_deployment_logs::*,
	update_deployment::*,
};
use crate::prelude::*;

/*
Figure out how to structure:
	- Volume
	- logs
	- metrics
	- backups
*/

/// The handler to setup all routes deployments. This will setup all the routes
/// for deployments. This includes creating, deleting, updating, and listing
/// deployments. It also includes starting, stopping, and streaming logs of
/// deployments.
#[instrument(skip(state))]
pub async fn setup_routes(state: &AppState) -> Router {
	Router::new()
		.merge(deploy_history::setup_routes(state).await)
		.mount_endpoint(machine_type, state)
		.mount_auth_endpoint(list_deployment, state)
		.mount_auth_endpoint(create_deployment, state)
		.mount_auth_endpoint(get_deployment_info, state)
		.mount_auth_endpoint(start_deployment, state)
		.mount_auth_endpoint(stop_deployment, state)
		.mount_auth_endpoint(get_deployment_logs, state)
		.mount_auth_endpoint(delete_deployment, state)
		.mount_auth_endpoint(update_deployment, state)
		.mount_auth_endpoint(get_deployment_metric, state)
		.mount_auth_endpoint(stream_deployment_logs, state)
}
