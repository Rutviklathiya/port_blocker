use libbpf_cargo::SkeletonBuilder;
use std::{
    fs::{create_dir, OpenOptions},
    io::{ErrorKind, Write},
    path::PathBuf,
};

const SRC: &str = "src/bpf/block.bpf.c";
const DST: &str = "block.skel.rs";
const VMLINUX_GEN_ARGS: &str = "btf dump file /sys/kernel/btf/vmlinux format c";
const BPFTOOL: &str = "bpftool";

fn generate_vmlinux(out_dir: &PathBuf) {
    println!("out is {:?}", out_dir);
    let args = VMLINUX_GEN_ARGS.split(' ').collect::<Vec<&str>>();
    let out = std::process::Command::new(BPFTOOL)
        .args(args)
        .output()
        .expect("could not generate vmlinux");

    let mut vmlinux_file = out_dir.clone();
    vmlinux_file.push("vmlinux.h");

    let mut vmlinux_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(vmlinux_file)
        .expect("could not open vmlinux.h file for write");
    vmlinux_file
        .write_all(&out.stdout)
        .expect("failed to write vmlinux file");
}

fn main() {
    let mut out = PathBuf::from("src/bpf/.out/");
    match create_dir(&out) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                panic!("Error creating out directory");
            }
        }
    }

    generate_vmlinux(&out);

    let clang_args = format!("-I {}", out.as_os_str().to_str().unwrap());

    out.push(DST);
    SkeletonBuilder::new()
        .source(SRC)
        .clang_args(clang_args)
        .build_and_generate(&out)
        .expect("failed to generate skeleton");
    println!("cargo:rerun-if-changed={}", SRC);
}
