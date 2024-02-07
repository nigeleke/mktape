use crate::args::{Args, Command, InputFile};
use crate::result::{Error, Result};

use std::fs::File;
use std::io::Write;

/// `mktape` provides the core functionality.
/// See main for `args` description. 
///
pub fn mktape(args: &Args) -> Result<()> {
    match &args.command {
        Command::Create { .. } => create(args),
        Command::List => list(args),
        Command::Extract { ..  } => extract(args),
    }
}

fn list(_args: &Args) -> Result<()> {
    Err(Error::NotImplemented)
}

fn create(args: &Args) -> Result<()> {
    const EOF: [u8; 4] = [0x00; 4];
    const EOT: [u8; 4] = [0xff; 4];

    let tape_path = &args.tape.path;
    let mut tap_file = File::create(tape_path)?;

    let inputs: Vec<InputFile> = Vec::new(); // TODO: Add back - &args.inputs;

    for input in inputs {
        let block_size = input.block_size;
        let packed_len = (block_size as u32).to_le_bytes();

        let path = &input.path;
        let path_name = path.display();

        let content = std::fs::read(path)?;
        let chunks = content.chunks(block_size);
        let chunks_len = chunks.len();
        for chunk in chunks {
            tap_file.write_all(&packed_len)?;
            tap_file.write_all(chunk)?;
            let padding = [0x00; 1].repeat(block_size - chunk.len());
            tap_file.write_all(&padding)?;
            tap_file.write_all(&packed_len)?;
        }
        tap_file.write_all(&EOF)?;

        println!("{}: {} bytes = {} records (blocksize {} bytes)",
            path_name, chunks_len * block_size, chunks_len, block_size);
    }

    Ok(tap_file.write_all(&EOT)?)
}

fn extract(_args: &Args) -> Result<()> {
    Err(Error::NotImplemented)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn error_when_cant_open_tap_file() {
        let args = "mktape ./pathdoesntexist/outfile.tap create infile1.txt";
        let args = Args::try_from(&args).expect("Expected successful parse");
        assert!(match mktape(&args) {
            Err(Error::Io(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn error_when_cant_open_input_file() { 
        let args = "mktape outfile.tap create infile1.txt";
        let args = Args::try_from(&args).expect("Expected successful parse");
        assert!(match mktape(&args) {
            Err(Error::Io(_)) => true,
            other => panic!("error_when_cant_open_input_file::unexpected error: {:?}", other),
        });
        // Housekeeping...
        std::fs::remove_file("outfile.tap").expect("error_when_cant_open_input_file: should be able to delete output file after test");
    }
}
