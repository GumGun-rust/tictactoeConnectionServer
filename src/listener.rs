use std::net::UdpSocket;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;


#[derive(Debug)]
pub enum MapAction{
    Add(u64, Sender<[u8; 64]>),
    Remove(u64)
}


pub fn listener_function(socket:UdpSocket, hashmap_control:Receiver<MapAction>) -> ! {
    use crate::client::SERVER_ADDRESS;
    
    let mut hash_map:HashMap<u64, Sender<[u8; 64]>> = HashMap::new();
    let _ = socket.send_to(&[0u8,0u8,0u8,2u8], SERVER_ADDRESS);
    let _ = socket.send_to(b"hola\n", SERVER_ADDRESS);
    
    loop {
        let mut buffer: [u8; 64] = [0;64];
        let recv_result = socket.recv(&mut buffer);
        
        if let Err(_) = recv_result {
            continue;
        }
        
        loop {
            match hashmap_control.recv_timeout(Duration::new(0, 1)) {
                Ok(data) => {
                    let key = match data {
                        MapAction::Add(key, sender) => {
                            hash_map.insert(key, sender);
                            key
                        },
                        MapAction::Remove(key) => {
                            hash_map.remove(&key);
                            key
                        },
                    };
                    println!("[listen] was deleted {:?}", key);
                },
                Err(err) => {
                    println!("[listen] Nothing to erase :\n\t{:?}", err);
                    break;
                }
            }
        }
        
        
        
        let key = u64::from_be_bytes(buffer[3..11].try_into().unwrap());
        println!("[listen] player key {} for message {:?}", key, buffer);
        
        match hash_map.get(&key) {
            Some(sender) => {
                let _ = sender.send(buffer);
            },
            None => {
                println!("[listen] no esta");
            }
        }
        //parse message to send it to corresponding message
    }
    
}
