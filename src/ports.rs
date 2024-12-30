use crate::common_ports::MOST_COMMON_PORTS_100;
use crate::model::Domain;
use crate::model::Ports;
use rayon::prelude::*;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::time::Duration;
pub fn scanports(mut subdomain: Domain) -> Domain {
    let socketaddress: Vec<SocketAddr> = format!("{}:1024", subdomain.dom_name)
        .to_socket_addrs()
        .expect("Scanning ports: Unexpected Error ")
        .collect();
    if socketaddress.is_empty() {
        return subdomain;
    }
    // println!("Socket Address : {:?}", &socketaddress);
    subdomain.ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scanport(socketaddress[0], *port))
        .filter(|port| port.is_open)
        .collect();
    subdomain
}

fn scanport(mut sock: SocketAddr, port: u16) -> Ports {
    let timeout = Duration::from_secs(3);
    sock.set_port(port);
    let is_open = TcpStream::connect_timeout(&sock, timeout).is_ok();
    Ports {
        is_open,
        port_name: port.to_string(),
    }
}
