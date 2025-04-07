use crate::prelude::*;

/// The 404 Not Found page. This page is shown when a route is not found.
#[component]
pub fn NotFoundPage() -> impl IntoView {
	view! {
		<ErrorPage title="Page Not Found">
			<Link
				r#type={LinkVariant::Link}
				style_variant={LinkStyleVariant::Contained}
				to="/"
			>
				{"Go to Home"}
			</Link>
		</ErrorPage>
	}
}
