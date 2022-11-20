use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    println!("Connected ");
    let result = stream.write("hello".as_bytes());
    let mut buf =[0;5];
    let result1 = stream.read(&mut buf);
    println!("Response from server :{:?}", str::from_utf8(&buf).unwrap());

}
