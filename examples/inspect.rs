use core::error;
use iso9660fs::volume::IsoFs;
use std::env::args;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = args().nth(1).unwrap_or("./test.iso".into());

    let mut fs = IsoFs::open(path)?;

    let license = fs.read_file("/GPL_3_0.TXT")?;
    println!("\nRead GPL_3_0.TXT ({} bytes)", license.len());
    println!("{}", str::from_utf8(license.as_slice())?);

    Ok(())
}
