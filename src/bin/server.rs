use std::thread;
use tungstenite::Message;

/// The local host config
const LOCAL_HOST: &str = "127.0.0.1:9001";

fn main() -> std::io::Result<()> {
    println!("This is server backend! ");

    let server = std::net::TcpListener::bind(LOCAL_HOST)?;
    for stream in server.incoming() {
        thread::spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                match msg {
                    Message::Text(text) => {
                        println!("msg in server: {:?}", text);
                    },
                    _ => {
                        unreachable!();
                    }
                }
            }
        });
    }

    Ok(())
}
