use std::net::{TcpListener, TcpStream};
use core::time::Duration;
use std::io::Write;
use std::io::Read;
use std::net::UdpSocket;
use std::sync::mpsc::{Sender,Receiver};

use aws_sdk_dynamodb as ddb;


use std::sync::mpsc::channel;

use crate::listener;


pub const SERVER_ADDRESS:&str = "44.199.207.136:50000";
pub const CONFIG_TABLE:&str = "configs";
pub const PLAYER_TABLE:&str = "players";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommandType{
    Create,
    Connect,
    //Disconnect,
    Move,
    //Delete,
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        use CommandType::*;
        match value {
            Create => 1,
            Connect => 2,
            //Disconnect => 3,
            Move => 4,
            //Delete => 5,
        }
    }
}

impl TryFrom<u8> for CommandType {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            _x @ 1 => Ok(CommandType::Create),
            _x @ 2 => Ok(CommandType::Connect),
            //_x @ 3 => Ok(CommandType::Disconnect),
            _x @ 4 => Ok(CommandType::Move),
            //_x @ 5 => Ok(CommandType::Delete),
            _ => Err("Not a valid Game Code")
        }
    }
}



pub fn start_server(port:&str) -> TcpListener {
    //4040
    let mut address = "0.0.0.0:".to_owned();
    address.push_str(port);
    TcpListener::bind(address).unwrap()
}


pub async fn handle_connection(mut connection:TcpStream, socket:UdpSocket, mut hashmap_control:Sender<listener::MapAction>, mut ddb_controler:ddb::Client) {
    
    
    let (player_id, board_id, receiver) = handle_validation(&mut hashmap_control, &mut connection, &mut ddb_controler).await;
    connection.write(format!("your player Id is {}\n", player_id).as_bytes());
    
    connection.set_read_timeout(Some(Duration::new(0, 100))).expect("timeout shoud be putted");
    connection.set_nodelay(true).expect("set_nodelay call failed");
    
    loop {
        //serv to client
        let serv_client_status = receiver.recv_timeout(Duration::new(0, 100));
        match serv_client_status {
            Ok(data) => {
                let prettify_response = response_to_string(&data);
                let _ = connection.write(prettify_response.as_bytes());
            },
            Err(err) => {
                //println!("{:?}", err);
            },
        }
        
        //client to serv
        let mut raw_client_buff = [0u8; 64];
        let mut command_buff = [0u8; 64];
        let raw_client_result = connection.read(&mut raw_client_buff);
        match raw_client_result {
            Ok(size) => {
                let client_buff = if size >= 1 {
                    &mut raw_client_buff[..size-1]
                } else {
                    &mut raw_client_buff
                };
                
                let (command_type, test, number0, number1) = match parse_command(&client_buff) {
                    Some(data) => data,
                    None => {
                        let socket_status = connection.write(b"incorrect command syntax second word should be a number\n");
                        if let Err(_) = socket_status {
                            break;
                        }
                        continue
                    },
                };
                match command_type {
                    CommandType::Create => {
                        let send_buff_size = build_create_command(&mut command_buff, player_id, board_id);
                        let comp_command_buff = &command_buff[..send_buff_size];
                        println!("[client] {:?}", comp_command_buff);
                        let socket_status = socket.send_to(comp_command_buff, SERVER_ADDRESS);
                        if let Err(err) = socket_status {
                            println!("[client] {:?}", err);
                            break;
                        }
                    }
                    CommandType::Connect => {
                        let send_buff_size = build_connect_command(&mut command_buff, player_id, board_id);
                        let comp_command_buff = &command_buff[..send_buff_size];
                        println!("[client] {:?}", comp_command_buff);
                        let socket_status = socket.send_to(comp_command_buff, SERVER_ADDRESS);
                        if let Err(err) = socket_status {
                            println!("[client] {:?}", err);
                            break;
                        }
                    },
                    CommandType::Move => {
                        let send_buff_size = build_move_command(&mut command_buff, player_id, board_id, number0.try_into().unwrap(), number1.try_into().unwrap());
                        let comp_command_buff = &command_buff[..send_buff_size];
                        println!("[client] {:?}", comp_command_buff);
                        let socket_status = socket.send_to(comp_command_buff, SERVER_ADDRESS);
                        if let Err(err) = socket_status {
                            println!("[client] {:?}", err);
                            break;
                        }
                    },
                }
            },
            Err(_err) => { /*println!("{:?}", _err);*/ }
        }
        
    }
    handle_client_close(player_id, &mut hashmap_control)
}

fn response_to_string(buff:&[u8]) -> String {
    let mut holder = "\n\n\n\n\n".to_owned();
    let player_id = u64::from_be_bytes(buff[3..11].try_into().expect("right value array"));
    let board_id = u64::from_be_bytes(buff[11..19].try_into().expect("right value array"));
    let won = buff[28];
    let turn = buff[1];
    holder.push_str(&format!("you are player {}\n", player_id));
    holder.push_str(&format!("test_print board_id {}\n", board_id));
    holder.push_str(&format!("won {}\n", won));
    match buff[1] {
        4 => {
            holder.push_str(&format!("game cant start until you invite other friend\n"));
        },
        _ => {
            holder.push_str(&format!("your turn {}\n", turn));
        }
    }
    
    //holder.push_str(&format!("your turn {}\n", turn));
    holder.push_str("\n\n  | 0 | 1 | 2 |\n");
    holder.push_str("--+---+---+---+--\n");
    holder.push_str(&format!("0 | {} | {} | {} |\n", buff[19], buff[20], buff[21]));
    holder.push_str("--+---+---+---+--\n");
    holder.push_str(&format!("1 | {} | {} | {} |\n", buff[22], buff[23], buff[24]));
    holder.push_str("--+---+---+---+--\n");
    holder.push_str(&format!("2 | {} | {} | {} |\n", buff[25], buff[26], buff[27]));
    holder.push_str("--+---+---+---+--\n  |   |   |   |\n");
    holder
}

fn parse_command(string_buffer:&[u8]) -> Option<(CommandType, u64, u8, u8)> {
    let command_string = String::from_utf8_lossy(string_buffer);
    let word_arr:Vec<_> = command_string.split(" ").collect();
    println!("[client] {:?}", word_arr);
    match word_arr[0] {
        "create" => {
            println!("[client {}] Create arrived", line!());
            Some((CommandType::Create, 0, 0, 0))
        },
        "connect" => {
            if word_arr.len() != 2 {
                return None;
            }
            println!("[client {}] Connect arrived", line!());
            let number:u64 = match word_arr[1].parse() {
                Ok(data) => {
                    data
                },
                Err(_) => {
                    return None
                }
            };
            Some((CommandType::Connect, number, 0, 0))
        },
        "move" => {
            if word_arr.len() != 3 {
                return None;
            }
            println!("[client {}] Move arrived", line!());
            let number0:u8 = match word_arr[1].parse() {
                Ok(data) => {
                    data
                },
                Err(_) => {
                    return None
                }
            };
            let number1:u8 = match word_arr[2].parse() {
                Ok(data) => {
                    data
                },
                Err(_) => {
                    return None
                }
            };
            Some((CommandType::Move, 0, number0, number1))
        }
        _ => {
            return None;
            //panic!();
        }
    }
}

fn build_create_command(send_buffer:&mut [u8], player_id:u64, board_id:u64) -> usize {
    send_buffer.fill(0);
    send_buffer[0] = u8::from(CommandType::Create); 
    send_buffer[1..9].clone_from_slice(&board_id.to_be_bytes()); 
    send_buffer[9..17].clone_from_slice(&player_id.to_be_bytes());
    8+8+1
}

fn build_connect_command(send_buffer:&mut [u8], player_id:u64, board_id:u64) -> usize {
    send_buffer.fill(0);
    send_buffer[0] = u8::from(CommandType::Connect); 
    send_buffer[1..9].clone_from_slice(&board_id.to_be_bytes()); 
    send_buffer[9..17].clone_from_slice(&player_id.to_be_bytes());
    8+8+1
}

fn build_move_command(send_buffer:&mut [u8], player_id:u64, board_id:u64, x_cord:u8, y_cord:u8) -> usize {
    send_buffer.fill(0);
    send_buffer[0] = u8::from(CommandType::Move); 
    send_buffer[1..9].clone_from_slice(&board_id.to_be_bytes()); 
    send_buffer[9..17].clone_from_slice(&player_id.to_be_bytes());
    send_buffer[17] = x_cord;
    send_buffer[18] = y_cord;
    8+8+1+2
}


async fn insert_player() -> u64 {
    panic!();
}

async fn get_next_player(ddb_client:&mut ddb::Client) -> u64 {
    use aws_sdk_dynamodb::types::AttributeValue::*;
    let command_holder = ddb_client.get_item();
    let command_holder = command_holder.table_name(CONFIG_TABLE);
    let command_holder = command_holder.key("config", S("player".to_owned()));
    let result_holder = command_holder.send().await;
    //println!("client {:#?}", hola);
    result_holder.unwrap().item().unwrap().get("number").unwrap().as_n().unwrap().parse::<u64>().unwrap()+1
}

async fn update_player(ddb_client:&mut ddb::Client, number:u64) {
    use aws_sdk_dynamodb::types::AttributeValue::*;
    let command_holder = ddb_client.put_item();
    let command_holder = command_holder.table_name(CONFIG_TABLE);
    let command_holder = command_holder.item("config", S("player".to_owned()));
    let _ = command_holder.item("number", N(number.to_string())).send().await.unwrap();
}

async fn get_next_board(ddb_client:&mut ddb::Client) -> u64 {
    use aws_sdk_dynamodb::types::AttributeValue::*;
    let command_holder = ddb_client.get_item();
    let command_holder = command_holder.table_name(CONFIG_TABLE);
    let command_holder = command_holder.key("config", S("board".to_owned()));
    let result_holder = command_holder.send().await;
    //println!("client {:#?}", hola);
    result_holder.unwrap().item().unwrap().get("number").unwrap().as_n().unwrap().parse::<u64>().unwrap()+1
}

async fn update_board(ddb_client:&mut ddb::Client, number:u64) {
    use aws_sdk_dynamodb::types::AttributeValue::*;
    let command_holder = ddb_client.put_item();
    let command_holder = command_holder.table_name(CONFIG_TABLE);
    let command_holder = command_holder.item("config", S("board".to_owned()));
    let _ = command_holder.item("number", N(number.to_string())).send().await.unwrap();
}

async fn handle_validation(hashmap_control:&mut Sender<listener::MapAction>, connection:&mut TcpStream, ddb_controler:&mut ddb::Client) -> (u64, u64, Receiver<[u8; 64]>) {
    //aws functions to validate
    
    let next_board = get_next_board(ddb_controler).await;
    println!("next board {}", next_board);
    update_board(ddb_controler, next_board).await;
    let next_player = get_next_player(ddb_controler).await;
    println!("next player {}", next_player);
    update_player(ddb_controler, next_player).await;
    
    let mut string = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    string.pop();
    let player_id = string.parse().unwrap();
    
    let _ = connection.write(b"0: for login\n1: for sign up\n");
    
    let mut buffer:[u8;1] = [0u8; 1];
    connection.read(&mut buffer);
    match buffer[0] {
        0 => {
            println!("log in");
        },
        1 => {
            println!("sign up");
        },
        _ => {
            println!("sign up");
            panic!("wrong answer");
        }
    }
    
    let (sender, receiver):(Sender<[u8; 64]>, Receiver<[u8; 64]>) = channel();
    
    println!("[client] key {} about to be send", player_id);
    let _send = hashmap_control.send(listener::MapAction::Add(player_id, sender));
    println!("[client] key {} was sended", player_id);
    
    (player_id, 100001, receiver)
}

fn handle_client_close(player_id:u64, hashmap_control:&mut Sender<listener::MapAction>) {
    println!("[client] key {} about to be send to deletion", player_id);
    let _ = hashmap_control.send(listener::MapAction::Remove(player_id));
    println!("[client] key {} was sended to deletion", player_id);
}

