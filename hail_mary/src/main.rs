use std::env;
use std::str;
use std::thread;
use std::net::UdpSocket;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let socket = match UdpSocket::bind("0.0.0.0:5514") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };

    let mut buf = [0; 2048];
    println!("About to start udp server");
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    println!("{}", str::from_utf8(&buf).unwrap_or(""));
                });
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
