//! Sunny Béta portfolio — an Axum server that renders Askama templates at
//! request time. Nothing is written to disk: no static HTML is generated.
//!
//! - [`content`] holds all site copy as data.
//! - [`templates`] maps Askama templates to Rust page types.
//! - [`routes`] wires the pages and the hand-written assets into a router.

pub mod content;
pub mod routes;
pub mod templates;

pub use routes::router;
