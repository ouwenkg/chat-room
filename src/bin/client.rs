use std::io::{Write, Read};

/// Message buffer
const MESSAGE_SIZE: usize = 32;
/// The local host config
const LOCAL_HOST: &str = "127.0.0.1:8080";

fn main() -> std::io::Result<()> {
    println!("*************************************");
    println!("*** Welcome to bear chat room *******");
    println!("*************************************");

    let mut stream = std::net::TcpStream::connect(LOCAL_HOST)?;
    loop {
        // Read line from stdin
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        buf = buf.trim_end_matches('\n').to_string();

        // Exit when input "exit"
        if buf == "exit" {
            break;
        } else {
            let mut msg_bytes = buf.as_bytes().to_vec();
            msg_bytes.resize(MESSAGE_SIZE, 0);
            stream.write(&msg_bytes);
            stream.flush();
        }
    }

    println!("*************************************");
    println!("*********** Bye, see you ************");
    println!("*************************************");
    Ok(())
}
