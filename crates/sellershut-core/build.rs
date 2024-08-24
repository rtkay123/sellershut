fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);

    let config = tonic_build::configure();

    let package = "categories";

    config
        .file_descriptor_set_path(out_dir.join(format!("{package}_descriptor.bin")))
        .compile_well_known_types(true)
        .compile(&["proto/category.proto"], &[""])?;

    Ok(())
}
