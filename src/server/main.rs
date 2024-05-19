use std::{env, io::{BufReader, Read, Write}, net::TcpListener};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Not enough args. Usage: ./server <agent_port> <public_port>");
    }
    let agent_port: u32 = args[1].parse().unwrap();
    let public_port: u32 = args[2].parse().unwrap();

    let agent_listener = TcpListener::bind(format!("127.0.0.1:{agent_port}")).unwrap();

    let mut agent_stream = match agent_listener.accept() {
        Ok((socket, addr)) => {
            println!("Agent connected: {addr}");
            socket
        },
        Err(e) => panic!("couldn't get client: {e:?}"),
    };

    let public_listener = TcpListener::bind(format!("127.0.0.1:{public_port}")).unwrap();
    for stream in public_listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buf_reader = BufReader::new(&mut stream);
        let mut buf: [u8; 512] = [0; 512];

        loop {
            match buf_reader.read(&mut buf) {
                Ok(0) => break, 
                Ok(bytes_read) => {
                    let _ = agent_stream.write(&buf[..bytes_read]);
                },
                Err(e) => {
                    eprintln!("Failed to read: {}", e);
                    break;
                }
            }
        }
    }


}
