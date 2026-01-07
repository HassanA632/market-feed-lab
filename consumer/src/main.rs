use std::io::{BufRead, BufReader};
use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:9000";
    let listener = TcpListener::bind(addr)?;
    println!("consumer listening on {}", addr);

    let (stream, peer) = listener.accept()?;
    println!("connected: {}", peer);

    // read lines from stream
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line?;
        println!("recv: {}", line);
    }

    println!("connection closed");
    Ok(())
}
