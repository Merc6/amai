mod lexer;
mod parser;
mod semantic_checker;
mod codegen;
mod vm;
mod common;
mod cli;
mod diagnostic;

use crate::cli::{Cli, Command};
use colored::Colorize;

fn main() {
    use clap::Parser;

    let cli = Cli::parse();
    if let Err(err) = run_cli(cli) {
        println!("{err}");
    }
}

pub fn run_cli(cli: Cli) -> Result<(), String> {
    use std::fs;

    let input = if let Command::Run { input } = cli.command {
        input.ok_or(format!("{}: Project runs are not supported yet", "error".bright_red().bold()))?
    } else {
        return Err(format!("{}: Unsupported command", "error".bright_red().bold()))
    };
    let contents = fs::read_to_string(&input)
        .map_err(|_| format!("{}: No such file: `{}`", "error".bright_red().bold(), input.italic()))?
        .replace("\r\n", "\n");

    let tokens = match lexer::lex(&input, &contents) {
        Ok(toks) => toks,
        Err(err) => {
            let lines = contents.lines().collect::<Vec<_>>();
            let line_starts = line_starts(&contents);
            return Err(err.display(&line_starts, &lines));
        },
    };

    let mut parser = parser::Parser::new(&input, &tokens);
    let mut ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            let lines = contents.lines().collect::<Vec<_>>();
            let line_starts = line_starts(&contents);
            return Err(
                err
                    .iter()
                    .map(|d| d.display(&line_starts, &lines))
                    .collect::<Vec<_>>().join("\n")
            );
        },
    };

    let mut sch = semantic_checker::SemanticChecker::new(ast.path.clone());

    sch.validate(&mut ast).map_err(|errors| {
        let lines = contents.lines().collect::<Vec<_>>();
        let line_starts = line_starts(&contents);
        errors
            .iter()
            .map(|d| d.display(&line_starts, &lines))
            .collect::<Vec<_>>().join("\n")
    })?;

    /*
    let mut bcg = codegen::BytecodeGenerator::new(&ast);
    let main_id = bcg.generate();
    let mut vm = bcg.prepare_vm();

    vm.call_function(main_id);
    vm.run().map_err(|err| format!("{}: {err}", "error".bright_red().bold()))?;
    vm.return_function();*/

    Ok(())
}

fn line_starts(s: &str) -> Vec<usize> {
    let mut indices = vec![0];
    
    for (i, ch) in s.char_indices() {
        if ch == '\n' {
            indices.push(i + 1);
        }
    }

    indices
}