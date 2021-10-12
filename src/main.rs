fn main() {
    let socket = String::from("127.0.0.1:80");
    let rusttp_server = Server::new(socket);
    rusttp_server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self{
            addr
        }
    }

    fn run(self) {
        println!("ouvindo no endereço {}", self.addr);
    }
}