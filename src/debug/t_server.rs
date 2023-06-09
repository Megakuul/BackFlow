use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read, Error};
use std::io::prelude::*;

pub async fn startup_testserver(bind_addr: &str) {

    async {
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
    }.await
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

        stream.write(format!("T_Server response from {}: {}", stream.local_addr()?, String::from_utf8_lossy(&buffer[..bytes_read])).as_bytes());
    }

    Ok(())
}