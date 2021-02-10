
use std::env;
use dummy_gen::{Result, validate_args};


fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    let (output_file_name, output_file_size) =  validate_args(&args)?;

    dummy_gen::write(output_file_name, output_file_size)?;

    println!("Sucesfully wrote {} bytes to {}", output_file_size, output_file_name.display());

    Ok(())
}
