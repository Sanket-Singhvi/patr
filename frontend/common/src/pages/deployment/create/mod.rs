use crate::prelude::*;

mod details;
mod head;
mod scale;

pub use self::{details::*, head::*, scale::*};

/// The Create Deployment Page
#[component]
pub fn CreateDeployment() -> impl IntoView {
	view! {
		<DeploymentPage>
			<CreateDeploymentHead />

			<ContainerBody class="gap-md overflow-y-auto px-md">
				<DeploymentDetails />
				// <ScaleDeployment />

				<div class="flex justify-end items-center gap-md w-full fit-wide-screen mx-auto mt-auto pt-md pb-xl px-md">
					<button
						type="submit"
						class="flex items-center justify-center btn btn-primary"
						// on:click={on_submit}
					>
						"CREATE"
					</button>
				</div>
			</ContainerBody>
		</DeploymentPage>
	}
}
