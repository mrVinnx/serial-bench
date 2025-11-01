use std::{fs, path::Path, process::Command};

fn main() {
    prost_build::Config::new()
        .out_dir("src/")
        .compile_protos(&["src/data.proto"], &["src/"])
        .unwrap();

    let schema = "src/data.fbs";
    let out_dir = Path::new("src");
    let target_name = "flat_data.rs";

    Command::new("flatc")
        .args(["--rust", "-o", "src", schema])
        .status()
        .expect("failed to run flatc");

    let generated = out_dir.join("data_generated.rs");
    let target = out_dir.join(target_name);
    if generated.exists() {
        fs::rename(&generated, &target).unwrap();
    }
}
