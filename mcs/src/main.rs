
use std::io::{Result};

pub mod mc_datatypes;
pub mod server;





fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host: String = "localhost".to_string();
        let port: u16 = 25565;
        let server = server::Server::new(host, port);
    }

    






    Ok(())

}
