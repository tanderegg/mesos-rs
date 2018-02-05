use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::os::unix::net::{SocketAddr};

//use std::rc::Weak;
//use process::Process;

#[derive(Debug, Clone)]
enum PIDAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    Unix(SocketAddr)
}

impl fmt::Display for PIDAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PIDAddress::V4(addr) => write!(f, "{}", addr),
            PIDAddress::V6(addr) => write!(f, "{}", addr),
            PIDAddress::Unix(ref sock) => write!(f, "{:?}", sock)
        }
    }
}

#[derive(Debug)]
pub struct PID {
    id: String,
    address: PIDAddress,
//    reference: Option<Weak<Process>>
}

impl Clone for PID {
    fn clone(&self) -> PID {
        PID {
            id: self.id.clone(),
            address: self.address.clone(),
//            reference: self.reference.clone()
        }
    }
}

impl PID {
    pub fn new() -> PID {
        PID {
            id: "EMPTY".to_string(),
            address: PIDAddress::V4(Ipv4Addr::new(0, 0, 0, 0)),
//            reference: None
        }
    }
}

impl fmt::Display for PID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.id, self.address)
    }
}
