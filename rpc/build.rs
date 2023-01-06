fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::compile_protos("proto/accounts.proto")?;
    tonic_build::compile_protos("proto/chat.proto")?;
    tonic_build::compile_protos("proto/media.proto")?;
    tonic_build::compile_protos("proto/timeline.proto")?;
    tonic_build::compile_protos("proto/user.proto")?;

    Ok(())
}