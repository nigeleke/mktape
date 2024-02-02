use crate::args::ValidatedArgs;
use crate::result::Result;

use std::fs::File;
use std::io::Write;

pub fn mktape(args: &[String]) -> Result<()> {
    let args = ValidatedArgs::from(args)?;

    const EOF: [u8; 4] = [0x00; 4];
    const EOT: [u8; 4] = [0xff; 4];

    let mut tap_file = File::create(args.tap_path())?;

    for spec in args.input_specs() {
        let block_size = spec.block_size();
        let packed_len = (block_size as u32).to_le_bytes();

        let content = std::fs::read(spec.path())?;
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
            spec.path().display(), chunks_len * block_size, chunks_len, block_size);
    }

    tap_file.write_all(&EOT)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::result::Error;

    #[test]
    fn error_when_cant_open_tap_file() {
        let args = "./pathdoesntexist/outfile.tap infile1.txt";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        assert!(match mktape(&args) {
            Err(Error::Io(_)) => true,
            _ => false,
        });
    }

    #[test]
    fn error_when_cant_open_input_file() { 
        let args = "outfile.tap infile1.txt";
        let args = Vec::from_iter(args.split(' ').map(String::from));
        assert!(match mktape(&args) {
            Err(Error::Io(_)) => true,
            _ => false,
        });
        // Housekeeping...
        std::fs::remove_file("outfile.tap").expect("error_when_cant_open_input_file: should be able to delete output file after test");
    }
}
