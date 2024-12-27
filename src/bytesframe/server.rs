use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{AnyDelimiterCodec, Framed};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addrs = "0.0.0.0:8088";

    let listener = TcpListener::bind(&addrs).await?;
    println!("lines server listening on: {}", addrs);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        println!("client: {} connected", client_addr);

        tokio::spawn(async move {
            match stream_handle(stream).await {
                Ok(_) => println!("client: {} disconnected", client_addr),
                Err(e) => eprintln!("client: {} error: {:?}", client_addr, e),
            }
        });
    }
}

pub async fn stream_handle(stream: TcpStream) -> anyhow::Result<()> {
    let mut framed = Framed::new(
        stream,
        AnyDelimiterCodec::new(b",;\n".to_vec(), b";".to_vec()),
    );

    loop {
        match framed.next().await {
            Some(Ok(line)) => {
                let item = String::from_utf8_lossy(&line);
                println!("received line: {}", item);

                let new_line = format!("echo: {:?}", item);

                println!("send new line: {}", new_line);
                framed.send(new_line).await?
            }
            Some(Err(e)) => return Err(e.into()),
            None => return Ok(()),
        }
    }
}
