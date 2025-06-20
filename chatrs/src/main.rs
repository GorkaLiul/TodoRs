use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::io;
#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0. 0.1:8080").await?;

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("NEW CONNECTION FROM: {addr}");


    }
    Ok(())
}


async fn process_connection(mut socket : TcpStream) {
    let data = b"does this work?";
    socket.write_all(data);
}
