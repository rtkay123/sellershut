#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
pub mod categories;

#[cfg_attr(
    docsrs,
    doc(cfg(
        all(feature = "tonic", feature = "categories"),
        // all(feature = "tonic", any(feature = "categories", feature = "users")),
    ))
)]
pub mod common;

#[cfg(feature = "categories")]
#[cfg_attr(docsrs, doc(cfg(feature = "categories")))]
pub mod google;
