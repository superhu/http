use std::io::Read;
use std::net::TcpListener;
use std::ptr::read;
use super::router::Router;
use http::httprequest::HttpRequest;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) ->Self {
        Self{
            socket_addr
        }
    }

    pub fn run(&self){
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut buffer = [0; 200];
            stream.read(&mut buffer).unwrap();
            let req: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::route(req, &mut stream);
        }
    }

}