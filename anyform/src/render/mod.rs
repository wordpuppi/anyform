//! Form rendering to different output formats.

mod json;
mod html;

#[cfg(feature = "tera")]
mod tera_render;

pub use html::{HtmlOptions, HtmlRenderer};
pub use json::{FormJson, JsonRenderer};

#[cfg(feature = "tera")]
pub use tera_render::TeraRenderer;
