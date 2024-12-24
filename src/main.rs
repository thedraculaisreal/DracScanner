use std::io::prelude::*;
use std::net::TcpStream;

fn main() std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let ip = args[1].clone();
    let mut port = 0;
    while port < 65535 {
	let mut stream = TcpStream::connect("{ip}:{port}")?;
    }

    Ok(())
}
