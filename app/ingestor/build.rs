fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &[
            "src/proto/log.proto",
        ],
        &["src"],
    )?;

    Ok(())
}
