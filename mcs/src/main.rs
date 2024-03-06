
use std::net;
use std::io::{Result};

pub mod mc_datatypes;





pub struct Server {
    host: String,
    port: u16,
    server_socket: net::UdpSocket,
}

impl Server {

    pub fn new(host: String, port: u16) -> Server {
                    

        Server {
            host: host, 
            port: port,
            server_socket: net::UdpSocket::bind({
                (host.to_string() + port.to_string())
            }).expect("uh oh spaghettio")

        }
    } 

}





fn main() ->  Result<()> {

    println!("Spooling Server...");


    






    Ok(())

}
