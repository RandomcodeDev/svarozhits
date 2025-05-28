use std::{fmt, fs, os::unix::process::CommandExt, path::PathBuf, process::Command};

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "System builder",
    about = "A tool to build the system software image"
)]
struct Args {
    #[arg(help = "target platform", default_value = "riscv64gc-unknown-none-elf")]
    target: String,
    #[arg(help = "target directory for the system workspace")]
    input: Option<PathBuf>,
    #[arg(help = "where to place the system image")]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let exe = std::env::current_exe().unwrap();

    let target = args.target;

    let input = match args.input {
        Some(input) => input,
        None => exe.with_file_name(format!("../../../system/target/{target}/debug")),
    };

    let output = match args.output {
        Some(out) => out,
        None => exe.with_file_name("../../../system.svp"),
    };

    println!("Creating system image in {:?} from {:?}", output, input);

    match fs::create_dir_all(&output) {
        Ok(_) => {}
        Err(err) => panic!("Failed to create output directory: {:?}", err),
    }

    let mut kernel_elf = input.clone();
    kernel_elf.push("svkernel.elf");
    let mut kernel_pe = output.clone();
    kernel_pe.push("svkernel.sys");

    let mut objcopy = Command::new(format!("riscv64-unknown-linux-musl-objcopy"));
    objcopy.args([
        "-O",
        "pei-riscv64-little",
        "--subsystem=xbox",
        kernel_elf.to_str().unwrap(),
        kernel_pe.to_str().unwrap(),
    ]);

    println!("Running {:?}", objcopy);
    match objcopy.output() {
        Ok(_) => {}
        Err(err) => panic!("objcopy failed: {:?}", err),
    }
}
