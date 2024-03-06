
use std::io::{Result};

pub mod mc_datatypes;
pub mod server;





fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host: String = "127.0.0.1".to_string();
        let port: u16 = 5000;
        let server = server::Server::new(host, port);
    }

    






    Ok(())

}
