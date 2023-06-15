mod runcmd;

use crate::{Result, SourceOption};
use clap::{Parser, Subcommand, ValueEnum};

/// sappho interpreter
#[derive(Debug, Parser)]
#[clap()]
pub struct Options {
    /// Turn on trace output
    #[clap(short, long)]
    pub trace: bool,

    #[clap(subcommand)]
    command: Command,
}

impl Options {
    pub fn parse() -> Self {
        <Options as Parser>::parse()
    }

    pub fn run(&self) -> Result<()> {
        use self::runcmd::RunCommand;

        self.cmd_run(self)
    }
}

/// subcommands
#[derive(Debug, Subcommand)]
#[clap()]
pub enum Command {
    /// Eval an input
    #[clap()]
    Eval(SourceOptions),

    /// Parse an input
    #[clap()]
    Parse(ParseOptions),
}

/// source options
#[derive(Debug, Parser)]
#[clap()]
pub struct SourceOptions {
    #[clap(default_value_t)]
    source: SourceOption,
}

/// parse options
#[derive(Debug, Parser)]
#[clap()]
pub struct ParseOptions {
    /// Select the parse output format
    #[clap(long, short, value_enum, default_value_t)]
    format: ParseFormat,

    #[clap(flatten)]
    source: SourceOptions,
}

/// parse output formats
#[derive(Clone, Debug, Default, ValueEnum)]
pub enum ParseFormat {
    /// The internal AST representation
    AST,

    /// Direct unparse
    Direct,

    /// The canonicalized source code
    #[default]
    Canonical,

    /// The reduced source code
    Reduced,
}
