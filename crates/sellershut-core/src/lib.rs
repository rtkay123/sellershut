#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations
)]

//! sellershut API utilities

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Categories API utilities
pub mod categories;

//#[cfg(all(feature = "tonic", any(feature = "categories")))] : more entities should come in `any`
#[cfg(all(feature = "tonic", feature = "categories"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "tonic", feature = "categories"))))]
/// Resuable utilities
pub mod common;
