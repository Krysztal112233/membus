use std::path::Path;

use socket2::{Domain, SockAddr, Socket, Type};

use crate::Error;

#[derive(Debug)]
pub struct SyncSocketAddr(Socket);

impl SyncSocketAddr {
    /// Construct [`SyncSocketAddr`] from [`AsRef<Path>`]
    ///
    /// # Error
    ///
    /// Mainly are [`::std::io::Error`]
    pub fn new<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let socket = Socket::new(Domain::UNIX, Type::STREAM, None)?;
        socket.bind(&SockAddr::unix(path)?)?;

        Ok(Self(socket))
    }

    /// Consume ownership and extract [`Socket`]
    pub fn into_inner(self) -> Socket {
        self.0
    }

    pub async fn send(&self) {}
}
