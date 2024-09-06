/// Pagination
#[cfg(any(feature = "categories"))]
pub mod pagination;

/// Utils
#[cfg(any(feature = "categories"))]
pub mod utils;

#[cfg(feature = "id-gen")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-gen")))]
/// ID Generation
pub mod id;
