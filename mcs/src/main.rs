use std::{
    io::{ prelude::*, BufReader, Result, Error},
    net::{TcpListener, TcpStream},
};
use std::fs;
use etherparse::*;

pub mod mc_datatypes;






fn main() ->  Result<()> {

    println!("Spooling Server...");

    {
        let host = "192.168.1.200";
        let port: u16 = 25565;
        let server = TcpListener::bind((host, port)).unwrap();
        
        loop {

            for stream in server.incoming() {
                println!("new client connected");
                handle_connection(stream.unwrap());
            }
        }



    }


    Ok(())

}


pub fn handle_connection( mut stream: TcpStream ) {
    let buf_reader = BufReader::new(&mut stream);

    let fuck = buf_reader.buffer();
    let size = fuck.len();
    println!("{}", size);
    
    



    

    
}
