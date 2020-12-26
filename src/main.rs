use async_std::prelude::*;
use async_std::net::{TcpListener, TcpStream};
use async_std::task;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Echo server is listening to {}", listener.local_addr()?);
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepted connection from {}", stream.peer_addr()?);
        let _handle = task::spawn(connect(stream));
    }

    Ok(())
}

async fn connect(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 1024];
    loop {
        // read data
        match stream.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => {
                // write data
                if let Err(e) = stream.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to the stream; err = {:?}", e);
                    break;
                }
            },
            Err(e) => {
                eprintln!("failed to read from the stream; err = {:?}", e);
                break;
            },
        }
    }
    Ok(())
}
