use std::net::{TcpListener, ToSocketAddrs};

pub struct SocketServer {
    pub tcp: TcpListener,
}

impl SocketServer {
    pub fn start_server<Addrs>(addr: Addrs) -> SocketServer
    where
        Addrs: ToSocketAddrs,
    {
        let temp = TcpListener::bind(addr);
        SocketServer { tcp: temp.unwrap() }
    }
}

impl std::fmt::Debug for SocketServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this is a socket")
    }
}
