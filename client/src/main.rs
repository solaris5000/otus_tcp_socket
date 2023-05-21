use std::{net::TcpStream, io::Write};
use sdtp::*;


fn main(){
    let mut client = TcpStream::connect("127.0.0.1:10001").unwrap();
    client.write_all(b"bytes");
}