use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "Svarozhits executable tool",
    about = "A tool to manipulate Svarozhits exectuables (SVEs)"
)]
struct Args {
    #[arg(short, long, help = "ELF input")]
    input: PathBuf,
    #[arg(short, long, help = "SVE output")]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    let output = args.output;

    println!("Converting {} to {}", input.to_str().unwrap(), output.to_str().unwrap());
    let elf_data = fs::read(input).expect("failed to read ELF");
    let object = object::File::parse(&*elf_data).expect("failed to parse ELF");
    let sve_data = svelib::build_sve(&object);
    fs::write(output, sve_data).expect("failed to write SVE");
}
