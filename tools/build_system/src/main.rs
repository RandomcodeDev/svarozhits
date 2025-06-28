use std::{fs, path::PathBuf, process::Command};

use clap::Parser;
use svelib::convert_object;

#[derive(Debug, Parser)]
#[command(
    name = "System builder",
    about = "A tool to build the system software image"
)]
struct Args {
    #[arg(short, long, help = "target platform", default_value = "riscv64gc-unknown-none-elf")]
    triple: String,
    #[arg(short, long, help = "target directory for the system workspace")]
    input: Option<PathBuf>,
    #[arg(short, long, help = "where to place the system image")]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);

    let exe = std::env::current_exe().unwrap();

    let triple = args.triple;

    let input = match args.input {
        Some(input) => input,
        None => exe.with_file_name(format!("../../../system/target/{triple}/debug")),
    };

    let output = match args.output {
        Some(out) => out,
        None => exe.with_file_name("../../../system.svp"),
    };

    println!("Creating system image in {:?} from {:?}", output, input);

    fs::create_dir_all(&output).expect("failed to create output directory");

    let mut kernel_elf = input.clone();
    kernel_elf.push("svkernel.elf");
    let mut kernel_dest = output.clone();
    kernel_dest.push("svkernel.elf");
    //let mut kernel_sve = output.clone();
    //kernel_sve.push("svkernel.sve");
    //convert_object(&kernel_elf, &kernel_sve);
    fs::copy(kernel_elf, kernel_dest).expect("failed to copy kernel to output directory");
}
