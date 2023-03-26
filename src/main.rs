use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read, Error};

use std::io::prelude::*;
use async_std::task;

#[cfg(debug_assertions)]
mod t_server;

fn main() {
    #[cfg(debug_assertions)]
    task::spawn(async {
        t_server::startup_testserver("127.0.0.1:5001").await;
    });
    
    let bind_addr = "127.0.0.1:5000";

    match init_tcp_listener(&bind_addr) {
        Ok(listener) => {
            println!("Listening on {}", bind_addr);

            for con in listener.incoming() {
                match con {
                    Ok(stream) => {
                        if let Err(e) = handle_connection(stream) {
                            eprintln!("Failed to establish connection: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Error accepting connection: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Failed to create TCP-Listener: {}", e),
    }
}

fn init_tcp_listener(bind_address: &str) -> Result<TcpListener> {
    let listener = TcpListener::bind(bind_address)?;
    Ok(listener)
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        println!("Received {} bytes from {}: {}", bytes_read, stream.peer_addr()?, String::from_utf8_lossy(&buffer[..bytes_read]));
        let mut redirect_target = TcpStream::connect("127.0.0.1:5001")?;
        redirect_target.write(&buffer[..bytes_read]);
    }

    Ok(())
}
