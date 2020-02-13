use std::io::Read;
use std::thread;

const MESSAGE_SIZE: usize = 32;

fn main() {
    println!("this is server");
    let listener = std::net::TcpListener::bind("127.0.0.1:8080").unwrap();
    // listener
    //     .set_nonblocking(true)
    //     .expect("Cannot set non-blocking");

    loop {
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("new client: addr {:?}", addr);
                println!("new client: stream {:?}", socket);

                thread::spawn(move || loop {
                    let mut buf = vec![0; MESSAGE_SIZE];
                    socket.read_exact(&mut buf).unwrap();
                    let message = buf.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                    println!("message bytes: {:?}", message);
                    let msg = String::from_utf8(message).unwrap();
                    println!("msg is {:?}", msg);
                });
            }
            Err(e) => println!("error {:?}", e),
        }
    }
}
