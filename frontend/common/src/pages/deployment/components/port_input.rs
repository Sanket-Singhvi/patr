use std::collections::BTreeMap;

use leptos::ev::Event;
use models::api::workspace::deployment::ExposedPortType;

use crate::prelude::*;

/// The Port Input, You can input port and type of port
#[component]
pub fn PortInput(
	/// Additional class names to apply to the outer div, if any.
	#[prop(into, optional)]
	class: Signal<String>,
	/// List of ports already present
	#[prop(into, optional, default = BTreeMap::new().into())]
	ports_list: Signal<BTreeMap<StringifiedU16, ExposedPortType>>,
	/// On Pressing Delete Button
	#[prop(into, optional, default = UnsyncCallback::new(|_| ()))]
	on_delete: UnsyncCallback<String>,
	/// On Pressing Add Button
	#[prop(into, optional, default = UnsyncCallback::new(|_| ()))]
	on_add: UnsyncCallback<(String, String)>,
) -> impl IntoView {
	let ports_list_store = StoredValue::new(ports_list.clone());
	let outer_class = format!("flex w-full {}", class.read());

	let port_number = RwSignal::new("".to_string());
	let port_type = RwSignal::new("".to_string());

	view! {
		<div class={outer_class}>
			<div class="w-1/6 flex justify-start items-center mb-auto mt-md">
				<label html_for="port" class="flex justify-start items-center">
					"Ports"
				</label>
			</div>

			<div class="w-5/6 flex flex-col items-start justify-start">
				<Show when={move || !ports_list.read().is_empty()}>
					<div class="flex w-full">
						<div class="w-full flex flex-col items-start justify-start">
							<For
								each={move || ports_list_store.with_value(|list| list.get())}
								key={|state| state.clone()}
								let:child
							>
								<div class="flex w-full mb-xs">
									<div class="w-5/12 pr-lg">
										<div class="w-full flex justify-start items-center px-xl py-sm br-sm bg-secondary-light">
											<span class="ml-md">{child.0.to_string()}</span>
										</div>
									</div>

									<div class="w-1/2">
										<div class="w-full flex justify-start items-center px-xl py-sm bg-secondary-light br-sm">
											<span class="px-sm">{child.1.to_string()}</span>
										</div>
									</div>

									<div class="w-1/12 flex items-center justify-center pl-sm">
										<button on:click=move |_| {
											on_delete.run(child.0.to_string());
										}>
											<Icon
												icon={IconType::Trash2}
												color={Color::Error}
												size={Size::Small}
											/>
										</button>
									</div>
								</div>
							</For>
						</div>
					</div>
				</Show>

				<form class="flex w-full">
					<div class="flex-5 flex flex-col justify-start items-start pr-lg gap-xxs">
						<Input
							value={Signal::derive(move || port_number.get())}
							on_input={move |ev: Event| {
								ev.prevent_default();
								port_number.set(event_target_value(&ev));
							}}
							r#type={InputType::Number}
							id="port"
							class="w-full"
							placeholder="Enter Port Number"
						/>

						// <Show when={move || {
						// 	store_error.with_value(|error| !error.get().clone().is_empty())
						// }}>
						// 	<Alert r#type={AlertType::Error} class="mt-xs">
						// 		{move || store_error.with_value(|error| error.get().clone())}
						// 	</Alert>
						// </Show>
					</div>

					<div class="flex-6 flex flex-col items-start justify-start gap-xxs">
						// <InputDropdown
						// 	value={port_type}
						// 	placeholder={"Select Protocol".to_string()}
						// 	options={exposed_port_types}
						// 	on_select={move |val: String| {
						// 		port_type.set(val);
						// 		if !port_type.get().is_empty() && !port_number.get().is_empty() {
						// 			on_add.call((port_number.get(), port_type.get()));
						// 		}
						// 	}}
						// />

					</div>

					<div class="flex-1 flex items-start justify-center">
						<Button
							variant={LinkStyleVariant::Contained}
							class="br-sm p-xs ml-md"
							on_click={move |_| {
								on_add.run((port_number.get(), port_type.get()))
							}}
						>
							<Icon icon={IconType::Plus} color={Color::Secondary} />
						</Button>
					</div>
				</form>
			</div>
		</div>
	}
}
