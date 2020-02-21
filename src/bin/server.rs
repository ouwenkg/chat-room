use std::thread;
use tungstenite::Message;

/// The local host config
const LOCAL_HOST: &str = "127.0.0.1:9001";

fn main() -> std::io::Result<()> {
    println!("This is server backend! ");

    let server = std::net::TcpListener::bind(LOCAL_HOST)?;
    for stream in server.incoming() {
        thread::spawn(move || {
            let stream = stream.unwrap();
            let connection_addr = stream.peer_addr().unwrap();
            println!("New connection: {:?}", connection_addr);
            let mut websocket = tungstenite::accept(stream).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                match msg {
                    Message::Text(text) => {
                        println!("from {:?}, message: {:?}", connection_addr, text);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        });
    }

    Ok(())
}
