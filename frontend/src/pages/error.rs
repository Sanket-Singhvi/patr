use crate::prelude::*;

/// The Error Page component. This component is used to show an error page
/// with a title and content.
#[component]
pub fn ErrorPage(
	/// The title of the error page
	#[prop(into)]
	title: String,
	/// The content of the error page
	#[prop(default = Box::new(|| ().into_any()))]
	children: Children,
) -> impl IntoView {
	view! {
		<div class="w-full h-full flex flex-col justify-start items-start bg-empty">
			<div class="w-full h-full flex flex-col items-center justify-start pt-[10rem] gap-md">
				<h2 class="text-primary text-2xl font-bold">{title}</h2>
				<div>
					{children()}
				</div>
			</div>
		</div>
	}
}
