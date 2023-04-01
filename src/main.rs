use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read};

use std::io::prelude::*;
use async_std::task;

mod conf;

#[cfg(debug_assertions)]
mod t_server;

#[cfg(debug_assertions)]
const DEBUG_ADDR: &str = "127.0.0.1:5001";

fn main() {
    #[cfg(debug_assertions)]
    task::spawn(async {
        t_server::startup_testserver(DEBUG_ADDR).await;
    });

    //TODO install File structure and default conf

    let config = conf::Configuration::new("/etc/backflow/conf.d/bf.ini");

    println!("The Path for certificates is {} and for redirects {}", config.certificates.path_certificates, config.redirects.path_enabled);
    
    let bind_addr: &str = "127.0.0.1:5000";

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
    let listener: TcpListener = TcpListener::bind(bind_address)?;
    Ok(listener)
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut redirect_target: TcpStream = TcpStream::connect("127.0.0.1:5001")?;

    let mut buffer: [u8; 4096] = [0u8; 4096];
    let mut response_buffer: [u8; 4096] = [0u8; 4096];
    
    loop {
        let bytes_read: usize = stream.read(&mut buffer)?;
        let response_bytes_read: usize = redirect_target.read(&mut response_buffer)?;

        if bytes_read == 0 {
            break;
        }

        //Redirect request from origin to target
        println!("\x1b[32mReceived {} bytes from {}: {}\x1b[0m", bytes_read, stream.peer_addr()?, String::from_utf8_lossy(&buffer[..bytes_read]));
        redirect_target.write(&buffer[..bytes_read]);
        
        //Redirect response from target back to origin
        stream.write(&response_buffer[..response_bytes_read]);
        println!("\x1b[33mReceived {} bytes from T_Server: {}: {}\x1b[0m", response_bytes_read, redirect_target.peer_addr()?, String::from_utf8_lossy(&response_buffer[..response_bytes_read]));
    }

    Ok(())
}
