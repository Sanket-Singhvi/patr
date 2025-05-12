use std::marker::PhantomData;

use axum_extra::routing::TypedPath;
use leptos::either::Either;
use leptos_router::{
	components::Route,
	hooks::{
		use_location,
		use_navigate,
		use_params as use_router_params,
		use_query as use_router_query,
	},
	params::Params,
	path,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::prelude::*;

/// A trait for types that can be used as a route in the application.
/// It also provides the path as well as the query parameters for the route.
pub trait TypedRoute:
	TypedPath + Params + DeserializeOwned + Serialize + PartialEq + Clone + Send + Sync + 'static
{
	/// Whether the route requires the user to be logged in.
	const REQUIRES_LOGIN: bool;

	/// The query parameters for the route.
	type Query: Params
		+ DeserializeOwned
		+ Serialize
		+ PartialEq
		+ Clone
		+ Default
		+ Send
		+ Sync
		+ 'static;
}

#[component(transparent)]
pub fn AppRoute<R, F, V>(
	/// Phantom data for the route
	#[prop(optional)]
	_phantom: PhantomData<R>,
	/// The view for the route
	view: F,
) -> impl IntoView
where
	R: TypedRoute,
	F: Fn(R::Query, R) -> V + Send + Sync + 'static,
	V: IntoView + 'static,
{
	let query: R::Query = use_router_query().get_untracked().unwrap_or_default();
	let params: R = use_router_params()
		.get_untracked()
		.expect("cannot parse params");
	let auth_state = expect_context::<AuthState>();
	let navigate = use_navigate();
	let location = use_location();

	Effect::new(move || {
		let params = use_router_query::<R::Query>().get();
		if params.is_err() {
			navigate(
				&format!(
					"{}{}",
					location.pathname.get(),
					serde_urlencoded::to_string(&R::Query::default()).unwrap()
				),
				Default::default(),
			);
		}
	});

	if R::REQUIRES_LOGIN {
		Either::Left(())
	} else {
		Either::Right(view! {
			<Route
				path=path!("users")
				view={move || view(query.clone(), params.clone())} />
		})
	}
}
