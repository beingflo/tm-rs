use std::env::Args;
use std::fs::File;
use std::io;
use std::fmt;

pub fn get_input(args: Args) -> Result<File, InputError> {
    let arg = args.skip(1).next().ok_or(InputError::NoArgument)?;
    Ok(File::open(arg)?)
}

pub enum InputError {
    IOError(io::Error),
    NoArgument,
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> Self {
        InputError::IOError(e)
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            InputError::IOError(ref e) => e.fmt(f),
            InputError::NoArgument => write!(f, "Provide a .tm file"),
        }
    }
}
