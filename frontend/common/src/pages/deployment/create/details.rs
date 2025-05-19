use crate::prelude::*;

/// The Deployment Details Page
#[component]
pub fn DeploymentDetails() -> impl IntoView {
	view! {
		<div class="flex flex-col items-start justify-start gap-md w-full fit-wide-screen px-xl mt-xl">
			<h4 class="text-white text-lg pb-md">"Deployment Details"</h4>

			<div class="flex flex-col items-start justify-start gap-md w-full h-full text-white">
				<div class="flex w-full">
					<div class="w-1/6 flex items-center justify-start">
						<label
							html_for="name"
							class="text-white text-sm flex items-center justify-start"
						>
							"Name"
						</label>
					</div>

					<div class="w-5/6 flex flex-col items-start justify-start">
						<Input
							placeholder="Deployment Name"
							r#type={InputType::Text}
							class="w-full"
							name="name"
							id="name"
						/>
					</div>
				</div>

				<div class="flex w-full">
					<div class="w-1/6 flex justify-start items-center">
						<label class="text-white text-sm flex justify-start items-center">
							"Registry"
						</label>
					</div>

					<div class="w-5/6 flex flex-col items-start justify-start">
						<Input
							placeholder="Registry Name"
							r#type={InputType::Text}
							class="w-full"
							value="Registry Name"
							name="registry_name"
							id="registry_name"
						/>
					</div>
				</div>

				<div class="flex w-full">
					<div class="w-1/6 flex justify-start items-center">
						<label class="text-white text-sm flex justify-start items-center">
							"Image Details"
						</label>
					</div>

					<div class="w-1/2 flex flex-col items-start justify-start">
						<Input
							placeholder="Enter Repository Image Name"
							r#type={InputType::Text}
							name="repository_name"
							class="w-full"
							id="repository_name"
						/>

					</div>

					<div class="w-2/6 pl-md flex flex-col items-start justify-start">
						<Input
							r#type={InputType::Text}
							placeholder="Choose Image Tag"
							class="w-full"
							name="image_tag"
							id="image_tag"
						/>
					</div>
				</div>

				<div class="flex w-full">
					<div class="w-1/6 flex justify-start items-center">
						<label class="text-white text-sm flex justify-start items-center">
							"Choose Runner"
						</label>
					</div>

					<div class="w-5/6 flex flex-col items-start justify-start">
						<Input
							r#type={InputType::Text}
							placeholder="Choose A Runner"
							class="w-full"
							name="runner"
							id="runner"
						/>
					</div>
				</div>
			</div>
		</div>


	}
}
