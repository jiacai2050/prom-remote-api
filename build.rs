use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["prompb/types.proto", "prompb/remote.proto"], &["prompb/"])?;
    Ok(())
}
