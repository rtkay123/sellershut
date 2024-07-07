#[cfg(all(feature = "tonic", feature = "categories"))]
tonic::include_proto!("common");

#[cfg(feature = "id-gen")]
#[cfg_attr(docsrs, doc(cfg(feature = "id-gen")))]
/// ID Generation
pub mod id_gen;
