use tungstenite::{connect, Message};
use url::Url;

/// The local host config
const LOCAL_HOST: &str = "ws://127.0.0.1:9001";

fn main() -> std::io::Result<()> {
    println!("*************************************");
    println!("*** Welcome to bear chat room *******");
    println!("*************************************");

    let (mut socket, _response) = connect(Url::parse(LOCAL_HOST).unwrap()).expect("Can't connect");

    loop {
        // Read line from stdin
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        buf = buf.trim_end_matches('\n').to_string();

        // Exit when input "exit"
        if buf == "exit" {
            break;
        } else {
            socket.write_message(Message::Text(buf.into())).unwrap();
        }

        // let msg = socket.read_message().expect("Error reading message");
        // println!("Received in client: {}", msg);
    }

    println!("*************************************");
    println!("*********** Bye, see you ************");
    println!("*************************************");
    Ok(())
}
