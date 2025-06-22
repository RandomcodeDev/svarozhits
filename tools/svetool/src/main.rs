use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use svelib::convert_object;

#[derive(Debug, Parser)]
#[command(
    name = "Svarozhits executable tool",
    about = "A tool to manipulate Svarozhits exectuables (SVEs)"
)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Convert {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Dump {
        file: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Command::Convert { input, output } => {
            svelib::convert_object(input, output);
        }
        Command::Dump { file } => {
            svelib::dump_sve(file);
        }
    }
}
