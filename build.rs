use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir);
    
    // Generate the protobuf files
    prost_build::compile_protos(
        &["proto/usdy_events.proto"],
        &["proto"],
    ).unwrap();
    
    // Copy the generated file to the src/pb directory
    let generated_file = out_path.join("usdy.types.v1.rs");
    let dest_file = PathBuf::from("src/pb/usdy.types.v1.rs");
    
    if generated_file.exists() {
        std::fs::copy(&generated_file, &dest_file).unwrap();
    }
    
    // Tell cargo to rerun this build script if the proto file changes
    println!("cargo:rerun-if-changed=proto/usdy_events.proto");
}
