use std::env;
use std::io::{self, Read, Write};
use std::net::{TcpStream};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Incorrect number of arguments. Usage: ./agent <server_addr:server_port> <redirect_port>");
    }
    let server_addr = &args[1];
    let redirect_port: u16 = args[2].parse().expect("Failed to parse redirect port as a number");

    let mut server_stream = TcpStream::connect(server_addr)
        .expect("Failed to connect to the server");
    println!("Connected to server at {}", server_addr);


    let mut local_stream = TcpStream::connect(("127.0.0.1", redirect_port))
        .expect("Failed to connect to localhost redirect port");
    println!("Connected to localhost on port {}", redirect_port);

    let mut buffer = [0; 512];
    loop {
        let len = server_stream.read(&mut buffer)?;
        if len == 0 {
            println!("Server closed the connection or no more data to read.");
            break;
        }

        local_stream.write_all(&buffer[..len])?;
    }

    Ok(())
}
