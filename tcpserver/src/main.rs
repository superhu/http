use std::io::{Read, Write};
use std::net::TcpListener;




fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000");

    for stream in listener.incoming() {
        let mut stream1 = stream.unwrap();
        println!("Connection established!");
        let mut buf = [0, 255];
        stream1.read(&mut buf).unwrap();
        stream1.write(&mut buf);

    }


}
