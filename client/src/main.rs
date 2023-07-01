use sdtp::client::read_responce;
use std::{io::*, net::TcpStream};

fn main() {
    let mut inp = String::new();
    println!("Welcome to SmartSocketTM(c) CLI.\nList of aviable commands:\nenbl - Enable socket\ndsbl - disable socket\npowr - get current power of socket\nstat - get current state of socket\nstop - close this CLI\n");
    let mut client = TcpStream::connect("127.0.0.1:10010").unwrap();
    loop {
        
        inp.clear();
        let _ = std::io::stdin().read_line(&mut inp);
        let input = inp.trim();
        if inp == *"stop\r\n" {
            break;
        }
        let _ = client.write_all(input.as_bytes());
        println!("{}", read_responce(&client));
    }
}
