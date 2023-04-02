mod conf;
mod listener;

#[cfg(debug_assertions)]
mod debug;

use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Result, Read};

use std::io::prelude::*;
use async_std::task;
use conf::conf::{Configuration, Certificates, Redirects};
use listener::tcp_listener;

fn main() {
    #[cfg(debug_assertions)]
    task::spawn(async {
        debug::t_server::startup_testserver("127.0.0.1:5001").await;
    });

    //TODO install File structure and default conf

    let config = Configuration::new("/etc/backflow/conf.d/bf.ini");

    println!("The Path for certificates is {} and for redirects {}", config.certificates.path_certificates, config.redirects.path_enabled);
    
    let bind_addr: &str = "127.0.0.1:5000";

    match tcp_listener::init_listener(&bind_addr) {
        Ok(listener) => {
            println!("Listening on {}", bind_addr);

            for con in listener.incoming() {
                match con {
                    Ok(stream) => {
                        if let Err(e) = tcp_listener::handle_connection(stream) {
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


