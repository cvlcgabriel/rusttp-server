use servidor::Server;
mod servidor;
mod http;

fn main() {
    let socket = String::from("127.0.0.1:80");
    let rusttp_server = servidor::Server::new(socket);
    rusttp_server.run();
}
