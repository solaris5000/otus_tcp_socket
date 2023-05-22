use std::{net::{TcpListener, TcpStream}, io::Read};

use sdtp::server::{self, SocketServer};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Socket {
    pub name: String,
    pub room_name: String,
    pub voltage: f32,
    pub amperage: f32,
    pub power: f32,
    pub enabled: bool,
    pub address : String,
    pub tcp : Option<SocketServer>
}

impl std::fmt::Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Socket: {} \nLocation: {}\nEnabled: {}\nAddress: {}",
            self.name,
            self.room_name,
            if self.enabled {
                format!("Yes\nPower: {}", self.power)
            } else {
                "No".to_owned()
            },
            self.address
        )
    }
}

impl Socket {
    pub fn new(name: &str) -> Self {
        Socket {
            name: (name.to_owned()),
            room_name: ("Unknown".to_owned()),
            voltage: (0.0),
            amperage: (0.0),
            power: (0.0),
            enabled: (false),
            address : "127.0.0.1:10001".to_owned(),
            tcp : Some(SocketServer::StartServer("127.0.0.1:10001")),
        }
    }

    pub fn listen(&self) -> impl Iterator<Item = TcpStream> + '_ {
        match &self.tcp {
            None => {panic!("there is no tcp server")},
            Some(ss) => {
                ss.tcp.incoming().map(|s| match s {
                    Ok(s) => {println!("Some command has been given"); s},
                    Err(e) => panic!("err"),
                })
            }
        }
    }

    fn scan_data(mut stream : TcpStream) {
        let buf = sdtp::read_command(&mut stream);
        println!("{}", buf);
    }
    

    pub fn _init(&mut self) {
        todo!();
    }

    pub fn _scan_power(&mut self) {
        todo!();
    }

    pub fn _scan_amperage(&mut self, _curr_a: f32) {
        todo!();
    }

    pub fn _scan_voltage(&mut self, _curr_v: f32) {
        todo!();
    }

    pub fn _get_power(&self) {
        println!("Current power is {} W", self.power);
    }
}

fn main() {
    let a = Socket::new("MySocket");
    println!("{}", a);
    let mut stream = a.listen();
    
    for msg in stream {
        Socket::scan_data(msg);
    }
}
