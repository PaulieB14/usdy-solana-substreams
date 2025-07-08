use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?); 
    
    tonic_build::configure()
        .build_server(false)
        .build_client(false)
        .out_dir(&out_dir)
        .compile(&["proto/usdy_events.proto"], &["proto"])?;
    
    Ok(())
}