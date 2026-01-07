use std::io::Write;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:9000";
    let mut stream = TcpStream::connect(addr)?;
    println!("connected to {addr}");

    for i in 0..10u64 {
        let msg = format!("{},BTC-USD,11111.00,1.112\n", i);
        stream.write_all(msg.as_bytes())?;
    }

    Ok(())
}
