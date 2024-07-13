#[cfg(all(feature = "tonic", any(feature = "categories", feature = "users")))]
tonic::include_proto!("common");

#[cfg(feature = "id-gen")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-gen")))]
/// ID Generation
pub mod id_gen;

#[cfg(all(
    feature = "tonic",
    any(feature = "rpc-server-categories", feature = "rpc-server-users")
))]
/// Pagination
pub mod paginate;
