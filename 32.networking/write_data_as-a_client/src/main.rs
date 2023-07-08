use std::io::{Read, Result, Write};

fn main() -> Result<()> {
    let mut stream = std::net::TcpStream::connect("0.0.0.0.12345")?;
    stream.write_all(&[0, 1, 2, 3])?;
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;
    println!("Recieved {:?}", buffer);

    Ok(())
}
