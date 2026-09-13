use std::{fs, io::Read, io::Result};

fn main() -> Result<()> {
    let mut file = fs::File::open("src/messages.txt")?;
    let mut buffer: [u8; 8] = [0; 8];

    loop {
        let bytes = file.read(&mut buffer)?;
        if bytes == 0 {
            break;
        };

        let current_data = &buffer[..bytes];
        let s = std::str::from_utf8(current_data).expect("invalid UTF-8");
        print!("{}", s);
    }
    println!();

    Ok(())
}
