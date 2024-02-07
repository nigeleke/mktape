pub(crate) mod args;
pub(crate) mod mktape;
pub mod result;

use crate::args::Args;
use crate::result::Result;

pub fn mktape(args: Vec<String>) -> Result<()> {
    let args = Args::from(args);
    println!("args: {:?}", args);

    mktape::mktape(&args)
}
