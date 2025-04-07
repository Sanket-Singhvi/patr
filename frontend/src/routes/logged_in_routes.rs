use std::any::Any;

use leptos_router::components::{Outlet, ProtectedParentRoute};

use crate::prelude::*;

/// Contains all the routes for when the user is logged in
#[component(transparent)]
pub fn LoggedInRoutes() -> impl IntoView {
	let (state, _) = AuthState::load();

	view! {
		<ProtectedParentRoute
			path={()}
			view={|| view! {
				<div class="fr-fs-fs full-width full-height bg-secondary">
					<Outlet />
				</div>
			}}
			redirect_path={|| "/login"}
			condition={|| Some(false)} >
			// <WorkspacedRoutes />
			// <NotWorkspacedRoutes />
			{}
		</ProtectedParentRoute>
	}
}
