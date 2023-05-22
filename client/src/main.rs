use std::{net::TcpStream, io::*};
use sdtp::{*, client::read_responce};


fn main(){
    
    let mut inp = String::new(); 
    while inp != "stop\r\n".to_owned() {
        inp.clear();
        let mut client = TcpStream::connect("127.0.0.1:10001").unwrap();
        let _ = std::io::stdin().read_line(&mut inp);
        let input = inp.trim();
        let _ = client.write_all(input.as_bytes());
        println!("{}", read_responce(&client));   
    }
}