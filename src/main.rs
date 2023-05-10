#[allow(unused_imports)]
use std::{thread, time};
use std::net::UdpSocket;

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};

mod client;
mod listener;
mod test;


fn main() {
    
    let new_client_listener = client::start_server("4040");
    
    let server_socket = UdpSocket::bind("127.0.0.1:9999").unwrap();
    
    //server_socket.send_to(b"incio\n", "127.0.0.1:4444");
    
    
    let (s_hashmap_control, r_hashmap_control):(Sender<listener::MapAction>, Receiver<listener::MapAction>) = channel();
    
    
    
    let listener_socket = server_socket.try_clone().unwrap();
    let _handler = thread::spawn(move || {
        listener::listener_function(listener_socket, r_hashmap_control);
    });
    
    loop{
        
        match new_client_listener.accept() {
            Ok((client_socket, addr)) => {
                
                //let local_socket = server_socket.try_clone().unwrap();
                //local_socket.send_to(b"preCreate thread\n", "127.0.0.1:4444");
                println!("thread created");
                println!("new client: {addr:?}");
                let hashmap_control_clone = s_hashmap_control.clone();
                let server_socket = server_socket.try_clone().unwrap();
                
                
                let _ = thread::spawn(move || {
                    client::handle_connection(client_socket, server_socket, hashmap_control_clone);
                });
                
            },
            Err(e) => {
                println!("couldn't get client: {e:?}");
            }
        }
        
    }
    /*
    thread::sleep(ten_millis);
    println!("Hello, world!");
    */
}

