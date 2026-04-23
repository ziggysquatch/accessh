use std::net::TcpStream;
use ssh2::Session;
use std::io::{self,Write};
use rpassword::read_password;
use std::io::prelude::*;
use std::io::{stdin,stdout};



pub fn get_connected(mut config: Config) -> Result<(), Box<dyn std::error::Error>> {    
    if config.host.is_empty() {
        let mut h = String::new();

        print!("Please input host: ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut h)
            .expect("Failed to read line");
        config.host = h;
    }
    if config.user.is_empty() {
        let mut u = String::new();

        print!("Username: ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut u)
            .expect("Failed to read line");
        config.user= u;
    }

    let connection = [&config.host.trim(),":22"].concat();
    println!("{connection}");
    // Connect to the local SSH server
    let tcp = TcpStream::connect(&connection).unwrap();
    let mut sess = Session::new().unwrap();
    // Assuming a session is already established
    // let algs = sess.supported_algs(ssh2::MethodType::Kex);
    // if let Ok(algs_list) = algs {
    //     for alg in algs_list {
    //         println!("Supported KEX: {}", alg);
    //     }
    // }
    
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();


    print!("Enter password: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let password = read_password().unwrap();

    sess.userauth_password(&config.user.trim(), &password).unwrap();
    //sess.set_blocking(false);
    println!("blah");
    let mut channel = sess.channel_session().unwrap();
    channel.request_pty("vt100", None, None).unwrap();

    channel.shell().unwrap();
    //channel.exec("bash -i")?;

    // let mut s = String::new();
    // channel.read_to_string(&mut s)?;
    // println!("{}", s);

    // let mut buf = [0; 1024];
    // while let Ok(n) = channel.read(&mut buf) {
    //     if n == 0 { break; }
    //     std::io::stdout().write_all(&buf[..n])?;
    // }
    // let mut buffer = [0u8; 4096];
    // let size = channel.read(&mut buffer).unwrap();
    // println!("{}", String::from_utf8_lossy(&buffer[..size]));

    let mut channel_stream = channel.stream(0);
    std::thread::spawn(move || {
        let mut buf = [0; 1024];
        while let Ok(n) = stdin().read(&mut buf) {
            if n == 0 { break; }
            let _ = channel_stream.write_all(&buf[..n]);
        }
    });

    let mut buf = [0; 1024];
    loop {
        let n = channel.read(&mut buf)?;
        if n == 0 { break; }
        stdout().write_all(&buf[..n])?;
        stdout().flush()?;
    }

    // if sess.authenticated() {
    //     println!("Successfully authenticated!");
    // } else {
    //     println!("didn't work");
    // }
    Ok(())
}

pub struct Config {
    user: String,
    host: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //if args.len() < 2 {
        //    return Err("need a host to accessh.");
        //}
        let mut host = String::new();
        let mut user = String::new();

        if args.len() > 1 && args[1].contains("@") {
            if let Some((u, h)) = args[1].split_once('@') {
                user = u.to_string();
                host = h.to_string();
                println!("user = {}", user);
                println!("host = {}", host);
            }
            
        } else if args.len() > 1 && !args[1].contains("@"){
            host = args[1].to_string();
            println!("host = {}", host);
        }
        Ok(Config { user, host })
    }
}