fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    let includes = [""];

    let config =
        tonic_build::configure().type_attribute(".", "#[derive(Eq, PartialOrd, Ord, Hash)]");

    #[cfg(feature = "serde")]
    let config = config.type_attribute(
        ".",
        "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all = \"snake_case\")]",
    );

    config
        .server_mod_attribute("categories", "#[cfg(feature = \"rpc-server-categories\")]")
        .client_mod_attribute("categories", "#[cfg(feature = \"rpc-client-categories\")]")
        .compile(&["proto/category.proto"], &includes)?;

    Ok(())
}
