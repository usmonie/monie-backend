fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/media.proto")?;
    tonic_build::compile_protos("proto/user.proto")?;
    tonic_build::compile_protos("proto/auth.proto")?;

    Ok(())
}