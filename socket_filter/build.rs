use std::fs::create_dir_all;
use std::path::Path;

use libbpf_cargo::SkeletonBuilder;

const PROG_DIR: &str = "./src/bpf";
const PROG_NAME: &str = "program";

fn main() {
    let prog_dir = Path::new(PROG_DIR);

    let out_dir = prog_dir.join(".output");
    create_dir_all(out_dir.as_path()).expect("Failed to create output directory");

    SkeletonBuilder::new(prog_dir.join(format!("{}.bpf.c", PROG_NAME)))
        .debug(true)
        .generate(out_dir.join(format!("{}.skel.rs", PROG_NAME)))
        .expect("BPF compilation failed");
}
