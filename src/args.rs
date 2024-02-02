use crate::result::{Error, Result};

use regex::Regex;

use std::iter::Iterator;
use std::path::PathBuf;

type BlockSize = usize;

#[derive(Debug, PartialEq)]
pub(crate) struct InputFileSpec {
    path: PathBuf,
    block_size: BlockSize,
}

impl InputFileSpec {
    pub(crate) fn path(&self) -> &PathBuf { &self.path }
    pub(crate) fn block_size(&self) -> BlockSize { self.block_size }
}

#[derive(Debug, PartialEq)]
pub(crate) struct ValidatedArgs {
    tap_path: PathBuf,
    input_specs: Vec<InputFileSpec>,
}

fn string_to_pathbuf(s: &str) -> Result<PathBuf> {
    Ok(PathBuf::new().join(s))
}

const MKTAPE_BLOCK_LENGTH_ENVVAR: &str = "MKTAPE_BLOCK_LENGTH";
const MKTAPE_BLOCK_LENGTH_DEFAULT: BlockSize = 1024;

fn string_to_input_spec(s: &str) -> Result<InputFileSpec> {
    let default_block_len = std::env::var(MKTAPE_BLOCK_LENGTH_ENVVAR)
        .map_or(Ok(MKTAPE_BLOCK_LENGTH_DEFAULT), |v| v.parse::<BlockSize>())?;

    fn reversed(input: &str) -> String {
        let reversed_chars: Vec<char> = input.chars().rev().collect();
        let reversed_string: String = reversed_chars.into_iter().collect();
        reversed_string
    }

    fn reversed_as_block_size(input: &str) -> Result<BlockSize> {
        let input = reversed(input);
        let block_size = input.parse::<BlockSize>()?;
        Ok(block_size)
    }

    let s = reversed(s);
    let regex = Regex::new("^(([^:]+):)?(.+)$").unwrap();
    let captures = regex.captures(&s).ok_or(Error::InvalidInputSpec)?;

    let path = captures.get(3).map(|m| reversed(m.as_str())).unwrap();
    let path = string_to_pathbuf(&path)?;

    let block_size = match captures.get(2).map(|m| reversed_as_block_size(m.as_str())) {
        Some(result) => { result? },
        None => { default_block_len },
    };

    Ok(InputFileSpec { path, block_size, })
}

impl ValidatedArgs {
    pub(crate) fn from(args: &[String]) -> Result<ValidatedArgs> {
        if args.len() <= 1 { return Err(Error::InvalidArgsCount) };
        let tap_path = string_to_pathbuf(&args[0])?;
        let Some(extension) = tap_path.extension() else {
            return Err(Error::InvalidTapeFilename)            
        };
        if !extension.eq("tap") { return Err(Error::InvalidTapeFilename) }
        let input_specs = args[1..].iter()
            .map(|s| string_to_input_spec(s))
            .collect::<Result<Vec<InputFileSpec>>>()?;
        Ok(ValidatedArgs { tap_path, input_specs })
    }

    pub(crate) fn tap_path(&self) -> &PathBuf { &self.tap_path }

    pub(crate) fn input_specs(&self) -> impl Iterator<Item = &InputFileSpec> {
        self.input_specs.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_with_zero_args() {
        let args = Vec::new();
        assert_eq!(ValidatedArgs::from(&args), Err(Error::InvalidArgsCount))
    }

    #[test]
    fn error_with_one_arg() {
        let args = "outfile.tap";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        assert_eq!(ValidatedArgs::from(&args), Err(Error::InvalidArgsCount))
    }

    #[test]
    fn error_when_tap_file_not_explicit() {
        let args = "outfile.err infile1.txt";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        assert_eq!(ValidatedArgs::from(&args), Err(Error::InvalidTapeFilename))
    }

    #[test]
    fn default_block_size_in_spec() {
        let args = "outfile.tap infile1.txt";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        let expected = ValidatedArgs {
            tap_path: PathBuf::new().join("outfile.tap"),
            input_specs: vec![
                InputFileSpec {
                    path: PathBuf::new().join("infile1.txt"),
                    block_size: 1024
                }
            ],
        };
        assert_eq!(ValidatedArgs::from(&args), Ok(expected))
    }

    #[test]
    fn environment_variable_block_size_in_spec() {
        std::env::set_var(MKTAPE_BLOCK_LENGTH_ENVVAR, "2048");
        let args = "outfile.tap infile1.txt";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        let expected = ValidatedArgs {
            tap_path: PathBuf::new().join("outfile.tap"),
            input_specs: vec![
                InputFileSpec {
                    path: PathBuf::new().join("infile1.txt"),
                    block_size: 2048
                }
            ],
        };
        let actual = ValidatedArgs::from(&args);
        std::env::remove_var(MKTAPE_BLOCK_LENGTH_ENVVAR);
        assert_eq!(actual, Ok(expected))
    }

    #[test]
    fn invalid_environment_variable_block_size_in_spec() {
        std::env::set_var(MKTAPE_BLOCK_LENGTH_ENVVAR, "abcd");
        let args = "outfile.tap infile1.txt";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        let actual = ValidatedArgs::from(&args);
        std::env::remove_var(MKTAPE_BLOCK_LENGTH_ENVVAR);
        assert_eq!(actual, Err(Error::InvalidInputSpec))
    }

    #[test]
    fn extracted_block_size_in_spec() {
        let args = "outfile.tap infile1.txt:4096";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        let expected = ValidatedArgs {
            tap_path: PathBuf::new().join("outfile.tap"),
            input_specs: vec![
                InputFileSpec {
                    path: PathBuf::new().join("infile1.txt"),
                    block_size: 4096
                }
            ],
        };
        assert_eq!(ValidatedArgs::from(&args), Ok(expected))
    }

    #[test]
    fn invalid_extracted_block_size_in_spec() {
        let args = "outfile.tap infile1.txt:abcd";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        assert_eq!(ValidatedArgs::from(&args), Err(Error::InvalidInputSpec))
    }


}