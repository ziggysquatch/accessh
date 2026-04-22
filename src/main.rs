use std::net::TcpStream;
use ssh2::Session;
use std::io;

fn main() {
    println!("Please input host: ");
    let mut host = String::new();
    
    io::stdin()
        .read_line(&mut host)
        .expect("Failed to read line");


let connection = [&host.trim(),":22"].concat();
println!("{connection}");
// Connect to the local SSH server
let tcp = TcpStream::connect(&connection).unwrap();
let mut sess = Session::new().unwrap();
sess.set_tcp_stream(tcp);
sess.handshake().unwrap();

sess.userauth_password("ziggysquatch", "zc2379").unwrap();
assert!(sess.authenticated());
}
