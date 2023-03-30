mod codegen;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

use std::{
    env,
    path::{Path, PathBuf},
};

use pico_args::Arguments;

const HELP: &str = "\
cargo-xtask

USAGE:
    cargo xtask <SUBCOMMAND>

SUBCOMMANDS:
    codegen     Generate code for AST
FLAGS:
    -h, --help    Prints help information
";

macro_rules! error {
    ($($tt:tt)*) => {
        Err(format!($($tt)*).into())
    }
}

pub(crate) use error;
use xshell::Shell;

#[derive(Debug)]
enum Subcommand {
    Codegen,
}

const RUN_WITH_HELP: &str = "Run with --help for more information.";

impl Subcommand {
    fn parse(mut args: Arguments) -> Result<Self> {
        let Some(subcommand) = args.subcommand()? else {
            return error!("expected a subcommand. {RUN_WITH_HELP}");
        };

        Ok(match subcommand.as_str() {
            "codegen" => Self::Codegen,
            _ => {
                return error!("unknown subcommand: {subcommand}. {RUN_WITH_HELP}");
            }
        })
    }

    fn run(self) -> Result<()> {
        let shell = Shell::new()?;
        shell.change_dir(project_root());
        match self {
            Self::Codegen => codegen::ast(),
        }
    }
}

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

fn main() -> Result<(), Error> {
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        println!("{}", HELP);
        return Ok(());
    }

    let subcommand = Subcommand::parse(args)?;

    subcommand.run()?;

    Ok(())
}
