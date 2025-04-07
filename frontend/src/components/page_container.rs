use crate::prelude::*;

/// A single page container, typically used for logged out Routes
#[component]
pub fn PageContainer(
	/// Additional class names to apply to the outer div, if any
	#[prop(into, optional)]
	class: Signal<String>,
	/// The contents of the page
	children: Children,
) -> impl IntoView {
	let class = move || {
		format!(
			"flex items-start justify-start bg-page-container w-full h-full bg-secondary {}",
			class.get()
		)
	};

	view! {
		<div class={class}>
			<main class="flex flex-col items-center justify-center w-full px-lg">
				{children()}
			</main>
		</div>
	}
}
