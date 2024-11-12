fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::compile_protos(&["protos/IR.proto"], &["protos/"])?;
    Ok(())
}
