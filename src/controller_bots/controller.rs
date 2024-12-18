

use tokio::sync::RwLock;

use crate::{ConnectionList, TcpListener, TcpStream, Arc, Error, Connection };

pub async fn handle_bots(socket: Arc<RwLock<TcpStream>>, connections: ConnectionList) {

    let bot_socket = socket.read().await;

    let socket_ip = match bot_socket.peer_addr() {
        Ok(info_socket) => info_socket.ip(),
        Err(_) => return
    };
    drop(bot_socket);

    println!("received connection from bot, ip : {socket_ip} ...");

    let storaged_sockets = connections.read().await;

    let not_is_a_unique_ip  = storaged_sockets
        .iter()
        .any(|info_socket| info_socket.ip == socket_ip);
    
    drop(storaged_sockets);

        if not_is_a_unique_ip {
            return
        }

        {
            let mut conns = connections.write().await;
            conns.push( Connection {
                ip:socket_ip,
                stream: Arc::clone(&socket)
            });
        }
    loop {

        async fn remove_connection(connections: &ConnectionList, socket: Arc<RwLock<TcpStream>>) {
            println!("bot desconnectd removing socket ...");
            let mut conns = connections.write().await;
            conns
                .retain(|socket_storage| !Arc::ptr_eq(&socket_storage.stream, &socket));
            drop(conns);
        }

        

        let mut sample_ping_buffer = [0;1];

        let bot_socket_try_read = socket.read().await;
        let try_read = bot_socket_try_read.try_read(&mut sample_ping_buffer);

        drop(bot_socket_try_read);

        match try_read {
            Ok(0) => {
                remove_connection(&connections, socket.clone()).await;
                break;
            },
            Ok(_) => {},
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {},
            Err(_) => {
                remove_connection(&connections, socket.clone()).await;
                break
            }
        }
    }


}
pub async fn bots_conections(host:&str, port:u32, connections: ConnectionList) -> Result<(), Box<dyn Error> > {

    let host_and_port = format!("{}:{}", host,port);
    let listen: TcpListener = TcpListener::bind(host_and_port.as_str()).await?;

    println!("Server listening connection from bots in {host_and_port}");

    loop {
        let (socket, _) = listen.accept().await?;

        let socket = Arc::new(RwLock::new(socket));
        let connections = Arc::clone(&connections);
    

        tokio::spawn(async move {
            handle_bots(socket, connections).await;
        });
    }
}