


use tokio::net::TcpStream;

use crate::utils::terminal::{reply, prompt,clear};

use super::terminal::default_prompt;


pub async fn view_menu(title: &str, itens_menu: &[&str], mut socket: &mut TcpStream) -> Result<u32, String> {
    clear(&mut socket).await;
    reply(title, true,&mut socket).await;
    
    for (current_index, item) in itens_menu.iter().enumerate() {
        
        let option: String = format!("[ {} ] - {} ", current_index + 1, item);
        reply(option.as_str() , true,&mut socket).await;
    }
    reply("[ 0 ] - exit c2 " , true,&mut socket).await;



    let option_selected: Result<u32, String> = match default_prompt(&mut socket).await {
        Ok(response_user) => {
            
            match response_user.trim().parse::<u32>() {
                Ok(number_parsed) => Ok(number_parsed), 
                Err(_) => Err("Invalid option !".to_string())
            }
        },
        Err(_e) => Err("Falied in recived input".to_string())
    };

    option_selected
  
}