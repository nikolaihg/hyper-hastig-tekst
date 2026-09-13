use std::{
    fs::File,
    io::{self, BufReader, ErrorKind::InvalidData, Read, Result},
};

fn main() -> Result<()> {
    let file = File::open("src/messages.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer: [u8; 8] = [0; 8];

    loop {
        let bytes = reader.read(&mut buffer)?;
        if bytes == 0 {
            break;
        };

        let value: &[u8] = &buffer[..bytes];

        match str::from_utf8(&value) {
            Ok(s) => print!("{s}"),
            Err(e) => {
                return Err(io::Error::new(
                    InvalidData,
                    format!("UTF-8 decoding failed: {e}"),
                ));
            }
        }
    }
    println!();

    Ok(())
}
