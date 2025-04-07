/// A link component that can be used to navigate to different pages. It can
/// be used as a button or a link.
mod link;
/// A single page container, typically used for logged out Routes
mod page_container;

pub use self::{link::*, page_container::*};
