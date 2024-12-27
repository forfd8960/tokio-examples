use futures::SinkExt;
use futures::StreamExt;

use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use tokio_util::codec::LinesCodec;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addrs = "127.0.0.1:8088";
    let stream = TcpStream::connect(&addrs).await?;

    let mut framed = Framed::new(stream, LinesCodec::new());

    for line in vec!["hello", "world"] {
        println!("send: {}", line);
        framed.send(line).await?;
    }

    while let Some(line) = framed.next().await {
        let line = line?;
        println!("recv: {}", line);
    }

    Ok(())
}
