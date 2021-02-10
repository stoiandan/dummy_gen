use std::fmt::{self, Display, Formatter};
use std::path::{Path};
use std::fs::{File};
use std::io::{BufWriter, prelude::*};
use rand::{ prelude::*};

#[derive(Debug)]
pub enum ArgsErr {
    NotEnoughArgs,
    NotNumber,
    IoError(std::io::Error)
}

impl Display for ArgsErr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match &self {
            ArgsErr::NotEnoughArgs => {
                "Not enough arguments provided. Please provide output file name."
            }
            ArgsErr::NotNumber => "Give size is not a number",
            ArgsErr::IoError(_) => "Could not open file for writing"
        };
        write!(f, "{}", message)
    }
}


impl From<std::io::Error> for ArgsErr {
    fn from(err : std::io::Error) -> Self {
        ArgsErr::IoError(err)
    }
}

pub type Result<T> = std::result::Result<T, ArgsErr>;

pub fn validate_args(args: &Vec<String>) -> Result<(&Path, u32)> {
    if args.len() < 2 {
        return Err(ArgsErr::NotEnoughArgs);
    }
    let num = is_number(&args[2])?;

    Ok((Path::new(&args[1]),num))
}

fn is_number(val: &String) -> Result<u32> {
    match val.parse::<u32>() {
        Ok(n) => Ok(n),
        _ => Err(ArgsErr::NotNumber),
    }
}


pub fn write(path : &Path, size : u32 ) -> Result<()>  {
    let file = File::create(&path)?;

    let mut buff_writer = BufWriter::new(&file);

    let mut buff : [u8; 1] = [0];

    for _ in 0..size {
        rand::thread_rng().fill_bytes(&mut buff);
        println!("Wrote {:?} to file",buff);
        buff_writer.write(&buff)?;
    }

    buff_writer.flush()?;

    Ok(())
}

