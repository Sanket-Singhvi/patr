use crate::{pages::deployment::components::PortInput, prelude::*};

/// A component that allows the user to set the running details of their
/// deployment
#[component]
pub fn RunningDetails() -> impl IntoView {
	view! {
		<div class="flex flex-col items-start justify-start w-full px-xl mt-xl text-white text-sm fit-wide-screen mx-auto gap-md">
			<h4 class="text-white text-lg pb-md">"Running Details"</h4>

			<div class="flex flex-col items-start justify-start h-full gap-xl w-full text-white">
				<PortInput />
			</div>
		</div>
	}
}
