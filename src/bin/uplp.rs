use std::{fmt::Debug, fs::File, io::{self, Read}, path::PathBuf};

use clap::{Parser, Subcommand};
use untitled_programming_language_project::{check_types, error, parse};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::AstDump { file } => {
            let expr = with_source_file(file, parse);
            handle_result(expr)
        }
        Commands::TypeCheck { file } => {
            let ty = with_source_file(file, check_types);
            handle_result(ty)
        }
    };
}

fn with_source_file<T>(p: PathBuf, op: fn(&str) -> Result<T, error::Error>) -> Result<T, Error> {
    let mut file = File::open(p)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(op(contents.as_str())?)
}

enum Error {
    Io(io::Error),
    Uplp(error::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<error::Error> for Error {
    fn from(e: error::Error) -> Error {
        Error::Uplp(e)
    }
}

fn handle_result<T: Debug>(r: Result<T, Error>) {
    match r {
        Ok(t) => println!("{:?}", t),
        Err(e) => {
            use Error::*;

            let msg = match e {
                Io(e) => format!("[IO Error] {:?}", e),
                Uplp(e) => format!("[Error] {:?}", e),
            };

            println!("{}", msg);
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Dump the AST of a uplp expression
    AstDump {
        /// The uplp source file to dump
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    /// Typecheck a uplp source file
    #[command(name = "typecheck")]
    TypeCheck {
        /// The uplp source file to typecheck
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
}
