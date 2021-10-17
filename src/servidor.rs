use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    endereço: String,
}

impl Server {
    pub fn new(endereço: String) -> Self {
        Self{
            endereço
        }
    }

    pub fn run(self) {
        println!("ouvindo no endereço {}", self.endereço);
        let listener = TcpListener::bind(self.endereço).unwrap();

        loop{
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    stream.read(&mut buffer);
                }
                Err(e) => { println!("Falhou em estabelecer uma conexão: {}", e); }
            }
        }
    }
}
