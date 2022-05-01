//use rusqlite::{params, Connection, Result};
//use futures::{stream, StreamExt};
use tokio::net::TcpStream;
//use std::io::{Read, Write};
use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

pub async fn connect(addrs: Vec<&str>) -> ()
{
    for address in addrs
    {
        let socket_address: SocketAddr = address
            .parse()
            .expect("Unable to parse socket address"); // TODO Handle Error in main
        ping_port(socket_address.ip(), socket_address.port(), 5).await;
    }
}

async fn ping_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);
    //println!("{} {}", target, port);
    match tokio::time::timeout(timeout, TcpStream::connect(&socket_address)).await 
    {
        Ok(Ok(stm)) => 
        {
            println!("Got response on: {}", port);
            _ = stm.try_write(b"test\n"); // return tuple with stuff
        },
        _ => {
            println!("Connection fail.");
        }
    }
}