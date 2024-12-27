use futures::SinkExt;
use futures::StreamExt;

use tokio::net::TcpStream;
use tokio_util::codec::AnyDelimiterCodec;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addrs = "127.0.0.1:8088";
    let stream = TcpStream::connect(&addrs).await?;

    let mut framed = Framed::new(
        stream,
        AnyDelimiterCodec::new(b",;\n".to_vec(), b";".to_vec()),
    );

    for line in vec!["hello", "world", "Nice", "to", "meet", "you"] {
        println!("send: {}", line);
        framed.send(line).await?;
    }

    while let Some(line) = framed.next().await {
        let line = line?;
        println!("recv: {}", String::from_utf8_lossy(&line));
    }

    Ok(())
}
