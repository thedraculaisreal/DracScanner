use std::env;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut ports = Vec::new();
    let ip = args[1].clone();
    let mut port = 20;
    while port < 65535 {
	if let Ok(stream) = TcpStream::connect("{ip}:{port}"){
	    println!("{port} is open");
	    ports.push(port);
	}
	else {
	    println!("{port} is closed");
	}
	port += 1;
    }

    println!("************************");

    for port in ports {
	println!("{port} is open")
    }

    Ok(())
}
