use clap::{Args as ClapArgs, Parser, Subcommand, ValueEnum};
#[cfg(test)]
use clap::error::Error as ClapError;
use regex::Regex;

use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

type BlockSize = usize;

#[derive(Debug, Parser, PartialEq)]
pub(crate) struct Args {
    /// The tape file to be used. [-f <FORMAT>] <FILE>, where <FORMAT> is `tap` or `backup`.
    #[command(flatten)]
    pub tape: TapeFile,

    #[command(subcommand)]
    pub command: Command,

}

impl Args {
    pub(crate) fn from(args: Vec<String>) -> Self {
        Args::parse_from(args)
    }

    #[cfg(test)]
    pub(crate) fn try_from(args: &str) -> Result<Self, ClapError> {
        let args = Vec::from_iter(args.split(' ').map(String::from));
        Args::try_parse_from(&args)
    }
}

#[derive(ClapArgs, Clone, Debug, PartialEq)]
/// The tape file to be used.
pub(crate) struct TapeFile {
    /// Tape file format
    #[arg(short, long, value_enum, default_value_t = TapeFormat::Tap)]
    pub format: TapeFormat,
    
    /// The `tap` or `backup` file to be processed
    #[arg(value_name = "TAPE")]
    pub path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
pub(crate) enum TapeFormat {
    Tap,
    Backup,
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub(crate) enum Command {
    /// Create a tape image from a set of input files.
    Create { 
        /// The input files to be added: <FILE>[:<BLOCKSIZE>]
        inputs: Vec<InputFile>,
    },
    /// List the contents of an existing tape image.
    List,
    /// Extract the contents of an existing tape image.
    Extract {
        /// Target folder for the extracted files.
        target: PathBuf,
    },
}

#[derive(ClapArgs, Clone, Debug, PartialEq)]
pub(crate) struct InputFile {
    pub path: PathBuf,
    pub block_size: BlockSize,
}

const MKTAPE_BLOCK_SIZE_ENVVAR: &str = "MKTAPE_BLOCK_SIZE";
const MKTAPE_BLOCK_SIZE_DEFAULT: BlockSize = 1024;

impl FromStr for InputFile {
    type Err = ParseIntError;
    fn from_str(item: &str) -> std::result::Result<Self, Self::Err> {
        let default_block_size = std::env::var(MKTAPE_BLOCK_SIZE_ENVVAR)
            .map_or(Ok(MKTAPE_BLOCK_SIZE_DEFAULT),
                    |v| v.parse::<BlockSize>())?;

        fn reversed(input: &str) -> String {
            let reversed_chars: Vec<char> = input.chars().rev().collect();
            let reversed_string: String = reversed_chars.into_iter().collect();
            reversed_string
        }

        let item = reversed(item);
        let regex = Regex::new("^(([^:]+):)?(.+)$").unwrap();
        let captures = regex.captures(&item).expect("mktape::args::Program Error in FromStr for InputFile");

        let path = captures.get(3).map(|m| reversed(m.as_str())).unwrap().into();
        let block_size = captures.get(2).map(|m| reversed(m.as_str()).parse()).transpose()?;
        let block_size = block_size.unwrap_or(default_block_size);

        Ok(InputFile { path, block_size, })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;

    #[test]
    fn error_with_zero_args() {
        let args = "mktape";
        let result = Args::try_from(&args).map_err(|e| e.kind());
        assert_eq!(result, Err(ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand))
    }

    #[test]
    fn use_default_block_size() {
        let args = "mktape outfile.tap create infile1.txt";
        let result = Args::try_from(&args).expect("Expected successful parse");
        assert_eq!(result.tape.path, PathBuf::new().join("outfile.tap"));
        let inputs = match result.command {
            Command::Create { inputs } => inputs,
            other => panic!("use_default_block_size::Unexpected command {:?}", other),
        };
        assert_eq!(inputs[0].path, PathBuf::new().join("infile1.txt"));
        assert_eq!(inputs[0].block_size, 1024);
    }

    #[test]
    fn use_environment_variable_block_size() {
        std::env::set_var(MKTAPE_BLOCK_SIZE_ENVVAR, "2048");
        let args = "mktape outfile.tap create infile1.txt";
        let result = Args::try_from(&args).expect("Expected successful parse");
        assert_eq!(result.tape.path, PathBuf::new().join("outfile.tap"));
        let inputs = match result.command {
            Command::Create { inputs } => inputs,
            other => panic!("use_default_block_size::Unexpected command {:?}", other),
        };
        assert_eq!(inputs[0].path, PathBuf::new().join("infile1.txt"));
        assert_eq!(inputs[0].block_size, 2048);
        std::env::remove_var(MKTAPE_BLOCK_SIZE_ENVVAR);
    }

    #[test]
    fn fail_on_invalid_environment_variable_block_size() {
        std::env::set_var(MKTAPE_BLOCK_SIZE_ENVVAR, "abcd");
        let args = "mktape outfile.tap create infile1.txt";
        let result = Args::try_from(&args).map_err(|e| e.kind());
        assert_eq!(result, Err(ErrorKind::ValueValidation));
        std::env::remove_var(MKTAPE_BLOCK_SIZE_ENVVAR);
    }

    #[test]
    fn use_input_block_size() {
        let args = "mktape outfile.tap create infile1.txt:4096";
        let result = Args::try_from(&args).expect("Expected successful parse");
        assert_eq!(result.tape.path, PathBuf::new().join("outfile.tap"));
        let inputs = match result.command {
            Command::Create { inputs } => inputs,
            other => panic!("use_default_block_size::Unexpected command {:?}", other),
        };
        assert_eq!(inputs[0].path, PathBuf::new().join("infile1.txt"));
        assert_eq!(inputs[0].block_size, 4096);
    }

    #[test]
    fn fail_on_invalid_input_block_size() {
        let args = "mktape outfile.tap create infile1.txt:abcd";
        let result = Args::try_from(&args).map_err(|e| e.kind());
        assert_eq!(result, Err(ErrorKind::ValueValidation));
    }
}