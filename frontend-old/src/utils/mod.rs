#[cfg(not(target_arch = "wasm32"))]
mod client;

#[cfg(not(target_arch = "wasm32"))]
pub use self::client::*;

mod hooks;
mod routes;
mod sidebar_items;
mod storage;

pub use self::{
	alignment::*,
	app_route::*,
	color::*,
	ext_traits::*,
	hooks::*,
	routes::*,
	sidebar_items::*,
	size::*,
	storage::*,
};
