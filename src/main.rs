use std::net::TcpStream;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let data = "*sensor1*GCHR6NVFBHK7XEGUSHVHSKWXAXVOKGHPKUZZEPY4XIB4TW3EB3GXVHO2,37.7749,-122.4194,30.0,50.0,90.0*sensor2*GDBDXBAQI5SZ7CCZPZT2UT4M72CJVQPV2WENRW2XBIJYX7IAUM6PCE3F,37.7750,-122.4195,31.0,51.0,91.0,*";
    stream.write_all(data.as_bytes())?;
    Ok(())
}
