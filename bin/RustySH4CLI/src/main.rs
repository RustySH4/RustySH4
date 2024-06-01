use clap::{Args, Parser, Subcommand};
use sh4utils::disasm::disasemble;
use std::io::Error;
use std::path::PathBuf;
use std::{fs, io::Write};

use sh4generator::{
    helper::save_and_format, impl_skeleton::generate_impl_empty, opcodes::generate_opcodes,
};

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "RustySH4 CLI is simple tool to generate opcodes, traits, implementations and many more for SH4 emulator and openCasio project"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// TODO: Add commands descriptions for help
#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    GenerateOpcodes(GenerateOpcodes),
    GenerateCpuSkeleton(GenerateCpuSkeleton),
    DisasmBinary(DisasmBinary),
}

#[derive(Args)]
struct GenerateOpcodes {
    path_to_html: PathBuf,
}

#[derive(Args)]
struct GenerateCpuSkeleton {
    path_to_html: PathBuf,
}

#[derive(Args)]
struct DisasmBinary {
    path_to_bin: PathBuf,
}

fn main() -> Result<(), Error> {
    // TODO: Add optional arg for out path
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateOpcodes(args) => {
            let path_to_html = args.path_to_html.clone();
            let html_content = fs::read_to_string(path_to_html.clone()).unwrap();

            let path_to_rs = path_to_html.parent().unwrap().join("opcodes.rs");

            save_and_format(path_to_rs, generate_opcodes(&html_content))?;
        }
        Commands::GenerateCpuSkeleton(args) => {
            let path_to_html = args.path_to_html.clone();
            let html_content = fs::read_to_string(path_to_html.clone()).unwrap();

            let path_to_rs = path_to_html.parent().unwrap().join("cpu_impl.rs");

            save_and_format(path_to_rs, generate_impl_empty(&html_content))?;
        }
        Commands::DisasmBinary(args) => {
            let path_to_bin = args.path_to_bin.clone();
            let bin_content: Vec<u8> = fs::read(path_to_bin.clone()).unwrap();

            let dissasembled = disasemble(bin_content);
            let path_to_out = path_to_bin.parent().unwrap().join("disasm.txt");

            let mut file = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(path_to_out)
                .unwrap();
            file.write_all(&dissasembled).unwrap();
        }
    };

    Ok(())
}
