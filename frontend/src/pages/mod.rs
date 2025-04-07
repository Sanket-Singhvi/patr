// /// The auth pages, including login and register.
// mod auth;
/// The error page component. This component is used to show an error page
/// with a title and content.
mod error;
/// The page that is shown when a route is not found.
mod not_found;

pub use self::{error::*, not_found::*};
