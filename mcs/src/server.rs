use std::{
    io::{ prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    host: String,
    port: u16,
    server_socket: TcpListener,
}

impl Server {

    pub fn new(host: String, port: u16) -> Server {
        let falure = host.clone();
        let tears = port.clone();
        let sock_addr = host + ":" + &port.to_string();
        
        let socket = TcpListener::bind(sock_addr);


        Server {
            host: falure, 
            port: tears,
            server_socket: socket.expect("uh oh spaghettio")

        }
    } 

    pub fn run(self) {
    
        loop {
            let mut client = self.server_socket.incoming();
            println!("new client connected");

            for stream in self.server_socket.incoming() {
                println!("new client connected");
                self.handle_connection(stream);
            }
        }

    }
    pub fn handle_connection(self, mut stream: TcpStream ) {
        let br = BufReader::new(&mut stream);
        let request: Vec<_> = br 
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        println!("Connection {:#?}", request);

        
    }

}
