//! See [`AmaiParser`].

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};

/// Parses command-line arguments into commands understood by the
/// Amai compiler.
#[derive(Debug, Parser)]
#[command(
    about = "The CLI for the Amai compiler",
    arg_required_else_help = true,
    version
)]
pub struct AmaiParser {
    /// The command to execute.
    #[command(subcommand)]
    command: AmaicCommand,
}

/// A possible subcommand that can be run by Amaic.
#[derive(Clone, Debug, Subcommand)]
pub enum AmaicCommand {
    /// Inspects the output at various stages of the compilation process.
    Inspect {
        /// Inspect the lexed output.
        #[arg(short, long)]
        lexed: bool,

        /// Inspect the parsed output.
        #[arg(short, long)]
        parsed: bool,

        /// Inspect the bytecode output.
        #[arg(short, long)]
        emitted: bool,

        /// The file to inspect.
        #[arg(value_hint = ValueHint::FilePath)]
        file: PathBuf,
    },

    /// Compiles and runs a given file.
    Run {
        /// The file to compile and run.
        #[arg(value_hint = ValueHint::FilePath)]
        file: PathBuf,
    },
}
