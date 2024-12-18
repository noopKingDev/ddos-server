
use std::error::Error;
use std::net::Shutdown;
use std::thread::sleep;
use std::time::Duration;

use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn clear(mut socket: &mut TcpStream) {
    reply("\x1b[2J\x1b[H", true, &mut socket).await;
}

pub async fn reply_with_delay(message: &str,delay: u64,mut socket: &mut TcpStream) {
    
    reply(message, true, socket).await;
    sleep(Duration::from_secs(delay));

}


pub async fn reply(message: &str,add_line_break: bool,mut socket: &mut TcpStream) {
    
    let message_converted: Vec<u8> = if add_line_break {
        let message_converted_in_string = format!("{message}\n");
            message_converted_in_string.into_bytes()
    } else {
        message.as_bytes().to_vec()
    };

    
    match socket.write_all(&message_converted).await {
        Ok(_) => {},
        Err(_) =>{
            socket.shutdown().await.unwrap();
        } 
    };
}

pub async fn prompt(message: &str, mut socket: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    reply(message, false,&mut socket).await;

    let mut response_user = [0; 1024];

    let recv_by_user = match socket.read(&mut response_user).await {
        Ok(0) => return Err("Conexão fechada".into()),
        Ok(n) => n,
        Err(e) => return Err(e.into()),
    };

    let response_string = String::from_utf8_lossy(&response_user[..recv_by_user]);

    Ok(response_string.to_string())
}

pub async fn default_prompt(mut socket: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    reply(" ~> ", false,&mut socket).await;

    let mut response_user = [0; 1024];

    let recv_by_user = match socket.read(&mut response_user).await {
        Ok(0) => return Err("Conexão fechada".into()),
        Ok(n) => n,
        Err(e) => return Err(e.into()),
    };

    let response_string = String::from_utf8_lossy(&response_user[..recv_by_user]);

    Ok(response_string.to_string())
}
