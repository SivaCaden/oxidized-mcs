use std::net;

pub struct Server {
    host: String,
    port: u16,
    server_socket: net::TcpListener,
}

impl Server {

    pub fn new(host: String, port: u16) -> Server {
        let falure = host.clone();
        let tears = port.clone();
        let sock_addr = host + ":" + &port.to_string();
        
        let socket = net::TcpListener::bind(sock_addr);


        Server {
            host: falure, 
            port: tears,
            server_socket: socket.expect("uh oh spaghettio")

        }
    } 

    pub fn run(self) {
    
        


    }

}
