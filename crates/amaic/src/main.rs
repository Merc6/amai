//! The CLI for the Amai compiler.

mod parser;

use anyhow::Ok;
use clap::Parser as _;

fn main() -> anyhow::Result<()> {
    let args = parser::AmaiParser::parse();
    _ = args;
    Ok(())
}
