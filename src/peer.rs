use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::message::Message; 

pub struct Peer {
    port: u16,
}

impl Peer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn gossip(&self, addr: String, period: Duration) {
        let port = self.port.clone();
        tokio::spawn(async move {
            loop {
                match TcpStream::connect(addr.clone()).await {
                    Ok(mut stream) => {
                        let msg = Message::new_random(8, port.to_string());
                        match stream.write_all(format!("{}({})", msg.content, msg.sender).as_bytes()).await {
                            Ok(()) => {
                                println!("Sent to {}: {}", addr, msg.content);
                            }
                            Err(e) => {
                                println!("Error writing to {}: {}", addr, e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to connect to {}: {}", addr, e);
                    }
                }
                tokio::time::sleep(period).await;
            }
        });
    }

    pub async fn listen(&self) {
        let listener = match TcpListener::bind(format!("127.0.0.1:{}", self.port)).await {
            Ok(listener) => listener,
            Err(e) => {
                println!("Failed to bind listener: {}", e);
                return; 
            }
        };

        loop {
            let (mut stream, _) = match listener.accept().await {
                Ok(result) => result,
                Err(e) => {
                    println!("Error accepting connection: {}", e);
                    continue; 
                }
            };

            tokio::spawn(async move {
                let mut buffer = [0; 128];
                loop {
                    match stream.read(&mut buffer).await {
                        Ok(0) => {
                            break;
                        }
                        Ok(bytes_read) => {
                            let received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
                            println!("Received: {}", received_message);
                        }
                        Err(e) => {
                            println!("Error reading from client: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    }
}

