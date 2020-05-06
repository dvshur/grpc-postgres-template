fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/proto")
        .compile(&["proto/notifier.proto", "proto/errors.proto"], &["proto"])?;

    Ok(())
}
