use leptos_router::components::A;
use web_sys::MouseEvent;

use crate::prelude::*;

/// Link Variant
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LinkVariant {
	/// A Normal Button. To be used with the Link Component
	#[default]
	Button,
	/// A Link. To be used with the Link Component
	Link,
}

/// The Type of Link to use. A contained link is a button with a background,
/// while a plain link looks like an anchor tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LinkStyleVariant {
	/// An Outlined Link. This is a button without a background, but with an
	/// outline.
	Outlined,
	/// A contained link. This is a button with a background.
	Contained,
	/// A plain link. This looks like an anchor tag.
	#[default]
	Plain,
}

/// Link component to navigate to other pages
/// Use the variant prop to switch between <a/> and <button/>
/// tag
#[component]
pub fn Link(
	/// Specifies which type of button to use, "button" or "submit", to be only
	/// used with the button variant
	#[prop(into, optional, default = false.into())]
	should_submit: Signal<bool>,
	/// The Target of the Link, to be used with the link variant
	#[prop(into, optional)]
	to: Signal<String>,
	/// Click Handler, to be only used with the button variant, this NEEDS
	/// JavaScript to be enabled.
	#[prop(optional)]
	on_click: Option<Callback<MouseEvent>>,
	/// The content of the Link
	#[prop(default = Arc::new(|| "".into_any()))]
	children: ChildrenFn,
	/// Additional class names to apply to the link, if any
	#[prop(into, optional)]
	class: Signal<String>,
	/// Color of the link
	#[prop(into, optional)]
	color: Signal<Color>,
	/// Button Variant i.e. a button or a Link. Defaults to Button
	#[prop(into, optional)]
	r#type: Signal<LinkVariant>,
	/// Variant of the Link
	#[prop(into, optional)]
	style_variant: Signal<LinkStyleVariant>,
	/// Whether the button is disabled or not
	#[prop(into, optional)]
	disabled: Signal<bool>,
) -> impl IntoView {
	let class = move || {
		format!(
			"flex items-center justify-center {} {}",
			class.get(),
			match style_variant.get() {
				LinkStyleVariant::Outlined => "btn-outline".to_string(),
				LinkStyleVariant::Contained => format!("btn btn-{}", color.get()),
				_ => format!("btn-plain text-{}", color.get()).to_string(),
			},
		)
	};

	let on_click = move |e: MouseEvent| {
		if let Some(click) = &on_click {
			e.prevent_default();
			click.run(e);
		}
	};

	let to = StoredValue::new(to);
	let children = StoredValue::new(children);

	move || match r#type.get() {
		LinkVariant::Link => Either::Left(view! {
			<div class={class}>
				<A href={move || to.with_value(|val| val.get())}>
					{children.with_value(|val| val())}
				</A>
			</div>
		}),
		LinkVariant::Button => Either::Right(view! {
			<button
				type={if should_submit.get() { "submit" } else { "button" }}
				on:click={on_click}
				disabled={move || disabled.get()}
				class={class}
			>
				{children.with_value(|val| val())}
			</button>
		}),
	}
}
