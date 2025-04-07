use std::any::Any;

use leptos_router::components::{Outlet, ProtectedParentRoute};

use crate::prelude::*;

/// Contains all the routes for when the user is logged in
#[component(transparent)]
pub fn LoggedOutRoutes() -> impl Any {
	view! {
		<ProtectedParentRoute
			path={}
			view={|| view! {
				<div class="flex items-start justify-start bg-page-container w-full h-full bg-secondary bg-image">
					<main class="flex flex-col items-center justify-center w-full px-lg">
						<Outlet />
					</main>
				</div>
			}}
			redirect_path={|| "/"}
			condition={|| Some(true)} >
			// <AppRoute<LoginRoute, _, _> view={|query, _| LoginForm(LoginFormProps { query })} />
			// <AppRoute<SignUpRoute, _, _> view={|query, _| SignUpForm(SignUpFormProps { query })} />
			// {app_type
			// 	.is_managed()
			// 	.then(|| {
			// 		view! { <AppRoute<ConfirmSignUp, _, _> view={ConfirmSignUpPage} /> }
			// 	})}
			{}
		</ProtectedParentRoute>
	}
}
