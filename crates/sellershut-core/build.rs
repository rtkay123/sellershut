enum Entity {
    Category,
}

impl Entity {
    fn package(&self) -> String {
        match self {
            Entity::Category => "categories",
        }
        .into()
    }
    fn path(&self) -> String {
        match self {
            Entity::Category => "proto/category.proto",
        }
        .into()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    let mut protos = vec![];

    if cfg!(feature = "categories") {
        protos.push(Entity::Category);
    }

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);

    for proto in protos {
        let path = proto.path();
        let package = proto.package();

        let config = tonic_build::configure();

        config
        .file_descriptor_set_path(out_dir.join(format!("{package}_descriptor.bin")))
            .server_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-server-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-server-{package}\")))]"),
            )
            .client_mod_attribute(
                &package,
                format!("#[cfg(feature = \"rpc-client-{package}\")] #[cfg_attr(docsrs, doc(cfg(feature = \"rpc-client-{package}\")))]"),
            )
        .compile_well_known_types(true)
        .compile(&[path], &[""])?;
    }

    Ok(())
}
