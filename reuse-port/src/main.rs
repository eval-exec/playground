use socket2::Domain;
use std::{
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

fn main() {
    std::thread::spawn(|| {
        let listener = spawn_listen("0.0.0.0:8115");
        let (mut stream, addr) = listener.accept().unwrap();
        let mut buf = String::new();
        stream.read_to_string(&mut buf).unwrap();
        println!("Received: {} from {}", buf, addr);
    });
    let stream1 = dial_tor("127.0.0.1:9050");
    let stream2 = dial_tor("127.0.0.1:9050");
    let stream3 = dial_tor("127.0.0.1:9050");
    std::thread::sleep(std::time::Duration::from_secs(10));
}

// dial_tor with reuse_addr, reuse_port
fn dial_tor(addr: &str) -> TcpStream {
    println!("dial tor...");
    let socket = socket2::Socket::new(Domain::IPV4, socket2::Type::STREAM, None).unwrap();
    let address: SocketAddr = addr.parse().unwrap();
    let address = address.into();
    socket.set_reuse_address(true).unwrap();
    socket.set_reuse_port(true).unwrap();
    socket.set_nonblocking(true).unwrap();
    {
        let listen_address: SocketAddr = "0.0.0.0:8115".parse().unwrap();
        let listen_address = listen_address.into();
        socket.bind(&listen_address).unwrap();
    }
    socket
        .connect_timeout(&address, Duration::from_secs(1))
        .unwrap();
    let mut stream: TcpStream = socket.into();
    stream
}

fn spawn_listen(addr: &str) -> TcpListener {
    // create a TCP listener
    let socket = socket2::Socket::new(Domain::IPV4, socket2::Type::STREAM, None).unwrap();

    let address: SocketAddr = addr.parse().unwrap();
    let address = address.into();
    socket.set_reuse_address(true).unwrap();
    socket.set_reuse_port(true).unwrap();
    socket.set_nonblocking(false).unwrap();
    socket.bind(&address).unwrap();
    socket.listen(128).unwrap();

    let listener: TcpListener = socket.into();
    listener
}
