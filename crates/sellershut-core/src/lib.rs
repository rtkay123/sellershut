//! Core
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations
)]

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
/// Categories
pub mod categories;

#[cfg(feature = "users")]
#[cfg_attr(docsrs, doc(cfg(feature = "users")))]
/// Users
pub mod users;

#[cfg(any(feature = "categories", feature = "users"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "categories", feature = "users"))))]
/// Common
pub mod common;

#[cfg(any(feature = "categories", feature = "users"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "categories", feature = "users"))))]
/// Common
pub mod google;
