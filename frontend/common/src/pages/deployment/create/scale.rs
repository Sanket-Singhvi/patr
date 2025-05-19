use crate::prelude::*;

/// A component that allows the user to scale their deployment
#[component]
pub fn ScaleDeployment() -> impl IntoView {
	view! {
		<div class="flex flex-col items-start justify-start w-full px-xl mt-xl text-white text-sm fit-wide-screen mx-auto gap-md">
			<h4 class="text-white text-lg pb-md">"Scale Your Servers"</h4>

			<div class="flex w-full">
				<div class="w-1/6 my-auto">
					<span class="text-sm">"Choose Horizontal Scale"</span>
				</div>

				<div class="w-5/6 flex flex-col justify-start items-start
					bg-secondary-light p-xl br-sm gap-md rounded-sm"
				>
					<p class="w-full tracking-[1px] text-xxs">
						"Choose the minimum and maximum number of instances for your deployment "
					</p>

					<div class="flex flex-col justify-start items-start gap-xl">
						<div
							style="width: 30%"
							class="w-full h-full flex justify-between items-center gap-xl"
						>
							<label class="flex-3" html_for="minHorizontalScale">
								"Minimum Scale"
							</label>
						</div>

						<div
							style="width: 30%;"
							class="w-full h-full flex justify-between items-center gap-xl"
						>
							<label class="flex-3" html_for="maxHorizontalScale">
								"Maximum Scale"
							</label>
						</div>
					</div>
				</div>
			</div>

			<div class="flex w-full">
				<div class="w-1/6 my-auto">
					<span class="text-sm">"Manage Resource Allocation"</span>
				</div>

				<div class="w-5/6 flex justify-start items-center overflow-auto">
					<div class="w-full p-xl rounded-sm bg-secondary-light flex flex-col items-start justify-start overflow-auto">
						<p class="tracking-[1px] mb-lg text-xxs">
							"Specify the resources to be allocated to your container"
						</p>

						<div class="flex justify-start items-center overflow-x-auto py-xxs gap-md">
						</div>
					</div>
				</div>
			</div>
		</div>
	}
}
