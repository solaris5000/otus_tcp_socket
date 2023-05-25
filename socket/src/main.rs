use sdtp::server::SocketServer;
use std::sync::RwLock;
use std::thread;
use std::{net::TcpStream, sync::Arc};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Socket {
    pub name: String,
    pub room_name: String,
    pub voltage: f32,
    pub amperage: f32,
    pub power: f32,
    pub enabled: bool,
    pub address: String,
}

pub struct TcpSocket {
    pub tcp: Option<SocketServer>,
}
#[allow(clippy::new_without_default)]
impl TcpSocket {
    pub fn new() -> TcpSocket {
        TcpSocket {
            tcp: (Some(SocketServer::start_server("127.0.0.1:10001"))),
        }
    }

    pub fn listen(&mut self) -> impl Iterator<Item = TcpStream> + '_ {
        match &self.tcp {
            None => {
                panic!("there is no tcp server")
            }
            Some(ss) => ss.tcp.incoming().map(|s| match s {
                Ok(s) => {
                    println!("Some command has been given");
                    s
                }
                Err(e) => panic!("err {:?}", e),
            }),
        }
    }

    fn scan_command(guard: Arc<RwLock<Socket>>, mut stream: &mut TcpStream) {
        let socket = guard.as_ref();
        let buf = sdtp::read_command(&mut stream);
        println!("CMD: {}", &buf);
        match &buf[..] {
            "powr" => {
                let socket = socket.read().unwrap();
                sdtp::send_command(b"F32D".to_owned(), &mut stream);
                if socket.enabled {
                    sdtp::send_command(socket.power.to_be_bytes(), &mut stream);
                } else {
                    sdtp::send_command(0f32.to_be_bytes(), &mut stream);
                }
            }
            "stat" => {
                let socket = socket.read().unwrap();
                sdtp::send_command(
                    if socket.enabled {
                        b"ebld".to_owned()
                    } else {
                        b"dbld".to_owned()
                    },
                    &mut stream,
                );
            }
            "enbl" => {
                let mut socket = socket.write().unwrap();
                socket.enabled = true;
                sdtp::send_command(b"enbl".to_owned(), &mut stream);
            }
            "dsbl" => {
                let mut socket = socket.write().unwrap();
                socket.enabled = false;
                sdtp::send_command(b"dsbl".to_owned(), &mut stream);
            }
            _ => {
                sdtp::send_command(b"E_WC".to_owned(), &mut stream);
            }
        }
        sdtp::send_command(b"R_OK".to_owned(), &mut stream);
    }
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
            power: (12.5),
            enabled: (false),
            address: "127.0.0.1:10001".to_owned(),
            //  tcp: Some(SocketServer::start_server("127.0.0.1:10001")),
        }
    }
    pub fn _init(&mut self) {
        todo!();
    }

    pub fn on(&mut self) {
        self.enabled = true;
    }

    pub fn off(&mut self) {
        self.enabled = false;
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
    let mut sa = TcpSocket::new();

    let a = Socket::new("MySocket");
    let arcs = Arc::new(RwLock::new(a));
    println!("{}", arcs.read().unwrap());

    let stream = sa.listen();

    for mut msg in stream {
        let acc = arcs.clone();
        thread::spawn(move || {
            TcpSocket::scan_command(acc, &mut msg);
            println!("doin soming");
        });
    }
}
