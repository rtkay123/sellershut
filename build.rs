fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    let includes = [""];

    tonic_build::configure()
        .server_mod_attribute("categories", "#[cfg(feature = \"rpc-server-categories\")]")
        .client_mod_attribute("categories", "#[cfg(feature = \"rpc-client-categories\")]")
        .compile(&["proto/category.proto"], &includes)?;

    Ok(())
}
