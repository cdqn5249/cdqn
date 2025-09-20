// src/runtime/network.rs

use crate::kernel::KDU;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

pub struct NodeServer;

impl NodeServer {
    /// Spawns the network server in a new thread.
    /// Returns a JoinHandle for the thread.
    pub fn spawn(addr: &str, kdu_tx: Sender<KDU>) -> JoinHandle<()> {
        let listener = TcpListener::bind(addr).expect("Failed to bind to network address");
        println!("[NodeServer] Thread started. Listening on {}", addr);

        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("[NodeServer] New connection: {}", stream.peer_addr().unwrap());
                        let kdu_tx_clone = kdu_tx.clone();
                        thread::spawn(move || {
                            handle_client(stream, kdu_tx_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("[NodeServer] Connection failed: {}", e);
                    }
                }
            }
        })
    }
}

/// The client handler now sends the received KDU back to the Orchestrator.
fn handle_client(mut stream: TcpStream, kdu_tx: Sender<KDU>) {
    let mut len_buffer = [0u8; 8];
    if stream.read_exact(&mut len_buffer).is_ok() {
        let kdu_len = u64::from_le_bytes(len_buffer);
        let mut kdu_buffer = vec![0; kdu_len as usize];

        if stream.read_exact(&mut kdu_buffer).is_ok() {
            let kdu: KDU =
                bincode::deserialize(&kdu_buffer).expect("Failed to deserialize KDU from stream");
            println!("[NodeServer] Received KDU with ID: {}. Sending to Orchestrator.", kdu.kdu_id);
            kdu_tx.send(kdu).expect("Failed to send KDU to orchestrator");
        }
    }
}

// NodeClient remains unchanged for now.
pub struct NodeClient;
impl NodeClient {
    pub fn connect(addr: &str) -> io::Result<TcpStream> {
        TcpStream::connect(addr)
    }
    pub fn send_kdu(stream: &mut TcpStream, kdu: &KDU) -> io::Result<()> {
        let kdu_bytes = bincode::serialize(kdu).expect("Failed to serialize KDU");
        let kdu_len = kdu_bytes.len() as u64;
        stream.write_all(&kdu_len.to_le_bytes())?;
        stream.write_all(&kdu_bytes)?;
        Ok(())
    }
}
