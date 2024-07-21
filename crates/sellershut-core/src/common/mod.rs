#[cfg(all(feature = "tonic", any(feature = "categories", feature = "users")))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "tonic", any(feature = "categories", feature = "users"))))
)]
/// Client-Server request types
pub mod request;

#[cfg(all(
    feature = "tonic",
    any(
        feature = "categories",
        feature = "users",
        feature = "rpc-client-categories",
        feature = "rpc-server-categories",
        feature = "rpc-client-users",
        feature = "rpc-server-users"
    )
))]
/// Pagination
pub mod pagination;

#[cfg(feature = "id-gen")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-gen")))]
/// ID Generation
pub mod id_gen;
