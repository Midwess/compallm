use socket2::{Domain, Protocol, Socket, Type};
use std::{
    io,
    net::{Ipv6Addr, SocketAddr},
};
use tokio::net::TcpListener;

pub async fn bind_tcp_listener(start: u16) -> io::Result<TcpListener> {
    if !(1..=50100).contains(&start) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "API starting port must be between 1 and 50100",
        ));
    }
    for port in start..=50100 {
        let socket = Socket::new(Domain::IPV6, Type::STREAM, Some(Protocol::TCP))?;
        socket.set_only_v6(false)?;
        #[cfg(unix)]
        socket.set_reuse_address(true)?;
        socket.set_nonblocking(true)?;
        match socket.bind(&SocketAddr::from((Ipv6Addr::UNSPECIFIED, port)).into()) {
            Ok(()) => {
                socket.listen(512)?;
                return TcpListener::from_std(socket.into());
            }
            Err(error) if error.kind() == io::ErrorKind::AddrInUse => continue,
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::AddrInUse,
        "No available API port in the configured range",
    ))
}
