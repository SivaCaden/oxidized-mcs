
use std::net;
use std::io::{Result};

pub mod mc_datatypes;





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

}
fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host: String = "127.0.0.1".to_string();
        let port: u16 = 5000;
        let server = Server::new(host, port);
    }

    






    Ok(())

}
