
#[cfg(test)]
mod test {
    use super::super::*;
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn test_creation() {
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        create_board(&board, &p0_socket, &p0_code);
    }
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn test_conection() {
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        create_board(&board, &p0_socket, &p0_code);
        connect_board(&board, &p1_socket, &p1_code);
        make_move(&board, &p0_socket, &p0_code, 0, 0);
    }
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn test_move_overlap() {
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        create_board(&board, &p0_socket, &p0_code);
        connect_board(&board, &p1_socket, &p1_code);
        make_move(&board, &p0_socket, &p0_code, 0, 0);
        make_move(&board, &p1_socket, &p1_code, 0, 0);
    }
    
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn p0_wins() {
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        
        create_board(&board, &p0_socket, &p0_code);
        connect_board(&board, &p1_socket, &p1_code);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
        make_move(&board, &p0_socket, &p0_code, 0, 0);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
        make_move(&board, &p1_socket, &p1_code, 1, 0);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
        make_move(&board, &p0_socket, &p0_code, 0, 1);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
        make_move(&board, &p1_socket, &p1_code, 1, 1);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
        make_move(&board, &p0_socket, &p0_code, 0, 2);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
        
        create_board(&board, &p0_socket, &p0_code);
        connect_board(&board, &p1_socket, &p1_code);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
    }
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn p1_wins() {
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        create_board(&board, &p0_socket, &p0_code);
        connect_board(&board, &p1_socket, &p1_code);
        make_move(&board, &p1_socket, &p1_code, 0, 0);
        make_move(&board, &p0_socket, &p0_code, 1, 0);
        make_move(&board, &p1_socket, &p1_code, 0, 1);
        make_move(&board, &p0_socket, &p0_code, 1, 1);
        make_move(&board, &p1_socket, &p1_code, 0, 2);
    }
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn post_start_connection() {
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        create_board(&board, &p0_socket, &p0_code);
        connect_board(&board, &p1_socket, &p1_code);
        receive_data(&p0_socket);
        make_move(&board, &p1_socket, &p1_code, 0, 0);
        make_move(&board, &p0_socket, &p0_code, 1, 0);
        make_move(&board, &p1_socket, &p1_code, 0, 1);
        let p1_socket = UdpSocket::bind("127.0.0.1:54411").unwrap();
        connect_board(&board, &p1_socket, &p1_code);
        receive_data(&p0_socket);
        receive_data(&p1_socket);
    }
    
    #[allow(dead_code, unused_variables)]
    #[ignore]
    #[test]
    fn individual_move(){
        let board = [1,0,0,0,0,0,0,0];
        let p0_socket = UdpSocket::bind("127.0.0.1:50010").unwrap();
        let p0_code = [1,0,0,0,0,0,0,0];
        let p1_socket = UdpSocket::bind("127.0.0.1:50011").unwrap();
        let p1_code = [2,0,0,0,0,0,0,0];
        make_move(&board, &p0_socket, &p0_code, 1, 2);
    }
    
    fn create_board(board:&[u8;8], socket:&UdpSocket, creator_id:&[u8;8]) {
        let mut buf:[u8;17] = [0;17];
        buf[0]=1u8;//.clone_from_slice(&[1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0]);
        buf[1..9].clone_from_slice(board);
        buf[9..17].clone_from_slice(creator_id);
        socket.send_to(&buf, "127.0.0.1:50000").expect("send failed");
    }
    
    
    fn connect_board(board:&[u8;8], socket:&UdpSocket, guest_id:&[u8;8]) {
        let mut buf:[u8;17] = [0;17];
        buf[0]=2u8;
        buf[1..9].clone_from_slice(board);
        buf[9..17].clone_from_slice(guest_id);
        socket.send_to(&buf, "127.0.0.1:50000").expect("send failed");
    }
    
    fn make_move(board:&[u8;8], socket:&UdpSocket, player_id:&[u8;8], x:u8, y:u8) {
        let mut buf:[u8;20] = [0;20];
        buf[0]=4u8;//.clone_from_slice(&[1,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0]);
        buf[1..9].clone_from_slice(board);
        buf[9..17].clone_from_slice(player_id);
        buf[17]=x;
        buf[18]=y;
        socket.send_to(&buf, "127.0.0.1:50000").expect("send failed");
    }
    
    fn receive_data(socket:&UdpSocket) {
        let mut buffer = [0; 64];
        let (amt, _src) = socket.recv_from(&mut buffer).expect("read should not crash");
        let buffer = &buffer[..amt];
        small_interpreter(buffer);
    }
    
    fn small_interpreter(raw_buffer:&[u8]) {
        //println!("{:?}", raw_buffer);
        match raw_buffer[0] {
            2 => {
                println!("connect");
                println!("\tturn {}",raw_buffer[1]);
                println!("\tstarted {}",raw_buffer[2]);
                //println!("player Id {}", raw_buffer[3..11]);
                for y in 0..3 {
                    println!("\t{} {} {}", raw_buffer[11+y*3], raw_buffer[12+y*3], raw_buffer[13+y*3]);
                }
            }
            4 => {
                println!("connect");
                println!("\tturn {}",raw_buffer[1]);
                println!("\tstarted {}",raw_buffer[2]);
                println!("player Id {}", raw_buffer[3]);
                for y in 0..3 {
                    println!("\t{} {} {}", raw_buffer[11+y*3], raw_buffer[12+y*3], raw_buffer[13+y*3]);
                }
                println!("won {}", raw_buffer[20]);
                
            }
            _ => {
                println!("posible error o falta de implementacion de printer");
            }
        }
        
    }
    
}
