extern crate threadpool;
extern crate itertools;

use itertools::Itertools;
use threadpool::ThreadPool;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    match args.len() {
        // no args passed -> run in client mode
        1 => {
            // Hardcoded vector of peer addresses
            let peers_addr: Vec<SocketAddr> = vec!["127.0.0.1:50000".parse().unwrap(),
                                                   "127.0.0.1:50001".parse().unwrap()];

            // Hashmap to store the connection to my peers
            let my_peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

            // Connect to my hardcoded peers
            connect_to_peers(peers_addr, my_peers.clone());

            // Run thread in an infinite loop sending messages to peers
            let cloned_peers = my_peers.clone();
            thread::spawn(|| {
                infinite_sender(cloned_peers);
            });

            // Launch server in well known port
            launch_server(50002);
        }

        // port passed -> run in server mode
        2 => {
            // Get port from command line
            let port = &args[1];

            // Cast port to integer
            if let Ok(port) = port.parse() {
                // Launch server
                launch_server(port);
            } else {
                help();
            }
        }

        // show help
        _ => {
            help();
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////
// HELP
//////////////////////////////////////////////////////////////////////////////////
fn help() {
    println!("Usage: ");
    println!("> cargo run [PORT]");
    println!("Run without parameters -> run in client mode, i.e. connect to servers in 50000 and 50001 ports and run server in port 50002");
    println!("Run with port -> run in server mode in given port");
}

//////////////////////////////////////////////////////////////////////////////////
// PEER CONNECTION FUNCTIONS
//////////////////////////////////////////////////////////////////////////////////
fn connect_to_peers(peers_addr: Vec<SocketAddr>, my_peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    // Try to connect to all my peers
    for peer_addr in peers_addr {
        connect_to_peer(peer_addr, my_peers.clone());
    }
}

fn connect_to_peer(addr: SocketAddr, my_peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    // Launch connection to the peer
    let stream = TcpStream::connect(addr).unwrap();

    // Insert in hashmap (addr as key, stream as value)
    my_peers.lock().unwrap().insert(addr, stream);
}

fn infinite_sender(my_peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    loop {
        // Send hello message to all my peers
        send_message_to_all_peers(String::from("hello\n").into_bytes(), my_peers.clone());

        // Make the thread sleep between hello and goodbye
        thread::sleep(Duration::from_millis(1000));

        // Send bye message to all peers
        send_message_to_all_peers(String::from("bye\n").into_bytes(), my_peers.clone());
    }
}

fn send_message_to_all_peers(msg: Vec<u8>, my_peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    // Create a thread pool to send messages to all peers
    // BEWARE: probably quite inefficient to do this here
    let pool = ThreadPool::new(8);

    // Send message to all peers (the sending to each peer will be performed in a different thread)
    for addr in my_peers.lock().unwrap().keys() {
        spawn_send_thread(&pool, &msg, addr, &my_peers);
    }
}

fn spawn_send_thread(pool: &ThreadPool, msg: &Vec<u8>, addr: &SocketAddr, my_peers: &Arc<Mutex<HashMap<SocketAddr, TcpStream>>>)
{
    // Clone all parameters for them to be moved to the thread
    let owned_msg = msg.clone();
    let owned_addr = addr.clone();
    let owned_peers = my_peers.clone();

    // Launch send message task in its own thread
    pool.execute(move || {
        send_message_to_peer(owned_msg, owned_addr, owned_peers);
    })
}

fn send_message_to_peer(msg: Vec<u8>, peer: SocketAddr, my_peers: Arc<Mutex<HashMap<SocketAddr, TcpStream>>>) {
    // Get map from arc
    let map = my_peers.lock().unwrap();

    // Get stream from arc
    let mut stream = map.get(&peer).unwrap();

    // Send message to peer
    stream.write_all(msg.as_ref()).unwrap();
}

//////////////////////////////////////////////////////////////////////////////////
// PEER CONNECTION FUNCTIONS
//////////////////////////////////////////////////////////////////////////////////
fn launch_server(port: u16) {
    // Bind the listener to the desired IP and port
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    // Create a thread pool to attend to
    let pool = ThreadPool::new(4);

    // Each incoming connection will be done in one thread
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // The client's socket address
        let _addr = stream.peer_addr().unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Extract addr
    let _addr = stream.peer_addr().unwrap();

    // Loop reading messages
    loop {
        let mut buffer = [0; 20];

        // Try to read
        match stream.read(&mut buffer) {
            Ok(num_read) => {
                println!("Received {}: {:02X}", num_read, buffer.iter().take(num_read).format(""));
                //println!("Received {:02X}", buffer.iter().format(""));
                //println!("Received {} bytes: {:?}", num_read, buffer);

                let response = String::from("pong\n");

                // Try to write
                match stream.write(response.as_bytes()) {
                    Ok(num_write) => {
                        println!("Written {} bytes as response", num_write);
                    }
                    Err(_e) => {
                        println!("Closed connection or something in write!");
                        break;
                    }
                }

                // Try to flush
                match stream.flush() {
                    Ok(_) => {
                        ();
                    }
                    Err(_e) => {
                        println!("Closed connection or something in flush!");
                        break;
                    }
                }
            }
            Err(_e) => {
                println!("Closed connection or something in read!");
                break;
            }
        }
    }
}
