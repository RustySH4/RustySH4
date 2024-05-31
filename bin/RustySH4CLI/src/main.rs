use clap::{Args, Parser, Subcommand};
use std::fs;
use std::io::Error;
use std::path::PathBuf;

use sh4generator::{helper::save_and_format, opcodes::generate_opcodes};

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
    DisasmBinary(DisasmBinary),
}

#[derive(Args)]
struct GenerateOpcodes {
    path_to_html: PathBuf,
}

// TODO: Add support for stdin
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

            let path_to_rs = path_to_html.parent().unwrap().join("opcodes_generated.rs");

            save_and_format(path_to_rs, generate_opcodes(&html_content))?;
        }
        Commands::DisasmBinary(_args) => {
            todo!()
        }
    };

    Ok(())
}
