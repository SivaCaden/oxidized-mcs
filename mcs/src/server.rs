use tokio::net::TcpListener;

pub struct Server {
    host: String,
    port: u16,
    server_socket: TcpListener,
}

impl Server {

    pub async fn new(host: String, port: u16) -> Server {
        let falure = host.clone();
        let tears = port.clone();
        let sock_addr = host + ":" + &port.to_string();
        
        let socket = TcpListener::bind(sock_addr).await;


        Server {
            host: falure, 
            port: tears,
            server_socket: socket.expect("uh oh spaghettio")

        }
    } 

    pub fn run(self) {
    
        loop {
            let mut client = self.server_socket.accept();
            println!("new client connected");



        }

    }

}
