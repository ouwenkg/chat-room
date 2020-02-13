use std::io::Write;

const MESSAGE_SIZE: usize = 32;

fn main() {
    println!("this is client");

    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();
    // stream.set_nonblocking(true).unwrap();

    loop {
        let mut buf = String::new();
        let input = std::io::stdin().read_line(&mut buf).unwrap();
        println!("input msg: {}", buf.trim_end_matches('\n'));

        if buf == "exit" {
            break;
        }

        let mut msg_bytes = buf.as_bytes().to_vec();
        msg_bytes.resize(MESSAGE_SIZE, 0);
        stream.write(&msg_bytes);
        stream.flush();
    }
}
