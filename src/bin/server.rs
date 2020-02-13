use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::thread;

/// Message buffer
const MESSAGE_SIZE: usize = 32;
/// The local host config
const LOCAL_HOST: &str = "127.0.0.1:8080";

fn main() -> std::io::Result<()> {
    println!("This is server backend! ");

    let listener = std::net::TcpListener::bind(LOCAL_HOST)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connection_addr = stream.peer_addr().unwrap();
                println!("New connection: {:?}", connection_addr);
                thread::spawn(move || handle_client(connection_addr, stream));
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}

pub fn handle_client(addr: SocketAddr, mut stream: TcpStream) {
    loop {
        let mut buf = vec![0u8; MESSAGE_SIZE];
        match stream.read_exact(&mut buf) {
            Ok(_) => {
                let message = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let msg = String::from_utf8(message).unwrap();
                println!("from {:?}, message: {:?}", addr, msg);
            }
            Err(e) => {
                println!("from {:?}, ncount error {:?}", addr, e);
                break;
            }
        }
    }
}
