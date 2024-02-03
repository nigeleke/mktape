use mktape::mktape::*;
use mktape::result::*;

const USAGE: &str = r#"

Usage: mktape <tapefilename.tap> <inputfilename:block_size>...

Note:
  1. <tapefilename.tap> must end with tap extension
  2. If <tapefilename.tap> already exists, then its contents will be overwritten.
  3. <inputfilename> optionally followed by ':<block_size>'
  4. Default block_size is defined by MKTAPE_BLOCK_SIZE environment variable,
     otherwise defaulted to 1024, if MKTAPE_BLOCK_SIZE is not defined.

"#;

/// Usage: mktape <tapefilename.tap> <inputfilename:block_size>...
///
/// Note:
///   1. <tapefilename.tap> must end with tap extension
///   2. If <tapefilename.tap> already exists, then its contents will be overwritten.
///   3. <inputfilename> optionally followed by ':<block_size>'
///   4. Default block_size is defined by MKTAPE_BLOCK_SIZE environment variable,
///      otherwise defaulted to 1024, if MKTAPE_BLOCK_SIZE is not defined.
///
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match mktape(&args) {
        Ok(_) => { Ok(()) },
        Err(e) => {
            println!("{}", USAGE);
            Err(e)
        }
    }
}
