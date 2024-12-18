use std::{error::Error,  sync::{Arc}};

use tokio::{net::TcpStream,sync::RwLock};

use crate::utils::terminal::{prompt, reply, reply_with_delay};

pub async fn shell(machine_socket : &Arc<RwLock<TcpStream>>, socket_user : &mut TcpStream ,ip_machine: &str) -> Result<(), Box<dyn Error>> {
    
    reply("connection establish !", true, socket_user).await;
    reply("use 'squit' to exit shell!", true, socket_user).await;

    loop {
        
        let title_prompt = format!("({ip_machine}@have) ~> ");
        let command = prompt(&title_prompt.as_str(), socket_user).await?;
        
        if command.contains("squit") {
            reply_with_delay("quiting shell ...", 1, socket_user).await;
            break
            }
        let mut socket = machine_socket.write().await;
        
        let escaped_command = escape_json_string(command.as_str());

        let json_payload = format!(r#"
                {{
                    "type_instruction":"shell",
                    "comand_shell": "{}"

                }}
                  "#, escaped_command);
        
        let response_machine = match prompt(&json_payload.as_str(), &mut socket).await {
            Ok(r) => r,
            Err(_) => {
               {

                //    reply("connection closing from machine !", true,  socket_user).await;
               }
                break
            }
        };
        drop(socket);
        reply(&response_machine, true, socket_user).await;
    }

    Ok(())
}

fn escape_json_string(input: &str) -> String {
    let mut escaped = String::new();
    for c in input.trim().chars() {
        match c {
            '"' => escaped.push_str("\\\""), // Escapa aspas
            '\\' => escaped.push_str("\\\\"), // Escapa barras invertidas
            '\n' => escaped.push_str("\\n"),   // Escapa novas linhas
            '\r' => escaped.push_str("\\r"),   // Escapa retornos de carro
            '\t' => escaped.push_str("\\t"),   // Escapa tabulações
            _ => escaped.push(c),              // Adiciona outros caracteres
        }
    }
    escaped
}