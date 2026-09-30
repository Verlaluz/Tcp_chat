use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Chat server started on port 8080!");

   
    let (server_transmitter, _server_receiver) = broadcast::channel::<String>(10);

    loop {
        let (mut client_socket, client_addr) = listener.accept().await.unwrap();
        println!("New client connected: {}", client_addr);

        
        let client_transmitter = server_transmitter.clone();
        let mut client_receiver = client_transmitter.subscribe();

        tokio::spawn(async move {
            
            let (socket_read_half, mut socket_write_half) = client_socket.split();
            
           
            let mut buffered_reader = BufReader::new(socket_read_half);
            let mut incoming_text_line = String::new();

            loop {
                tokio::select! {
                   
                    read_result = buffered_reader.read_line(&mut incoming_text_line) => {
                        if read_result.unwrap() == 0 {
                            println!("Client {} disconnected.", client_addr);
                            break; 
                        }
                        
                        let formatted_msg = format!("{}: {}", client_addr, incoming_text_line);
                        let _ = client_transmitter.send(formatted_msg);
                        
                        incoming_text_line.clear();
                    }
                    
                  
                    broadcast_result = client_receiver.recv() => {
                        let broadcasted_msg = broadcast_result.unwrap();
                        let _ = socket_write_half.write_all(broadcasted_msg.as_bytes()).await;
                    }
                }
            }
        });
    }
}