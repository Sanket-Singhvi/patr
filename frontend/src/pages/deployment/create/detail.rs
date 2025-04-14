use crate::prelude::*;

/// The Deploy Details Page, has stuff like name, runner, registry, etc.
#[component]
pub fn DeploymentDetails() -> impl IntoView {
	view! {
		<div class="flex flex-col items-start justify-start w-full fit-wide-screen px-xl mt-xl">
			<h4 class="text-red-500 text-xl pb-md">"Deployment Details"</h4>

			<div class="flex flex-col items-start justify-start w-full h-full text-white">
				<div class="flex mb-xs w-full">
					<div class="flex-2 flex items-center justify-start">
						<label
							html_for="name"
							class="text-white text-sm flex items-center justify-start"
						>
							"Name"
						</label>
					</div>

					<div class="flex-10 flex flex-col items-start justify-start">
						<Input
							placeholder="Deployment Name"
							r#type={InputType::Text}
							class="w-full"
							name="name"
							id="name"
							value={"Deployment Name"}
						/>
					</div>
				</div>

				<div class="flex my-xs w-full mb-md">
					<div class="flex-2 flex justify-start items-center">
						<label class="text-white text-sm flex justify-start items-center">
							"Registry"
						</label>
					</div>

					<div class="flex-10 flex flex-col items-start justify-start">
						// TODO: Add Registry Input and Input Dropdown
						"Registry"
					</div>
				</div>

				<div class="flex my-xs w-full">
					<div class="flex-2 flex justify-start items-center">
						<label class="text-white text-sm flex justify-start items-center">
							"Image Details"
						</label>
					</div>

					<div class="flex-6 flex flex-col items-start justify-start">
						<Input
							placeholder="Enter Repository Image Name"
							r#type={InputType::Text}
							name="repository_name"
							class="w-full"
							id="repository_name"
							value={"Image Details"}
						/>
					</div>

					<div class="flex-4 pl-md flex flex-col items-start justify-start">
						<Input
							r#type={InputType::Text}
							placeholder="Choose Image Tag"
							class="w-full"
							name="image_tag"
							id="image_tag"
							value={"Image Tag"}
						/>
					</div>
				</div>
			</div>
		</div>
	}
}
