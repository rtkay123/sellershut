/// Pagination
#[cfg(all(
    feature = "tonic",
    any(
        feature = "categories",
        feature = "rpc-client-categories",
        feature = "rpc-server-categories",
    )
))]
pub mod pagination;

/// Utils
#[cfg(all(
    feature = "tonic",
    any(
        feature = "categories",
        feature = "rpc-client-categories",
        feature = "rpc-server-categories",
    )
))]
pub mod utils;

#[cfg(feature = "id-gen")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-gen")))]
/// ID Generation
pub mod id;
