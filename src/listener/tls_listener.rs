use std::io::{Read, Write, Result};
use std::net::{TcpListener, TcpStream};
use async_std::task;
use native_tls::{Identity, TlsAcceptor, TlsStream};

pub fn init_listener(bind_address: &str) -> Result<TcpListener> {
    let listener: TcpListener = TcpListener::bind(bind_address)?;
    Ok(listener)
}

pub fn handle_connection(acceptor: TlsAcceptor, mut stream: TcpStream) -> Result<()> {
    let mut tls_stream = acceptor.accept(stream).unwrap();

    let mut buffer: [u8; 4096] = [0u8; 4096];

    loop {
        let bytes_read: usize = tls_stream.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        println!("Received {} bytes from {}: {}",bytes_read, tls_stream.get_ref().peer_addr()?, String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}