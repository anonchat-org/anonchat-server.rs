use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{
    env,
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpListener, TcpStream},
};

type SocketMap = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;

fn handle_client(mut stream: TcpStream, clients: SocketMap) {
    loop {
        let mut buffer = [0; 2048];

        let len = stream.read(&mut buffer).unwrap();

        if len == 0 {
            clients.lock().unwrap().remove(&stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }

        for client in clients.lock().unwrap().values_mut() {
            client.write_all(&buffer[..len]).unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: anonchat-server <host:port>");
        println!("Example: anonchat-server 0.0.0.0:6969");
        return;
    }

    let clients: SocketMap = Arc::new(Mutex::new(HashMap::new()));
    let server = TcpListener::bind(&args[1]).unwrap();

    for client in server.incoming() {
        let clients = clients.clone();
        let client = client.unwrap();

        clients
            .lock()
            .unwrap()
            .insert(client.peer_addr().unwrap(), client.try_clone().unwrap());
        thread::spawn(|| handle_client(client, clients));
    }
}
