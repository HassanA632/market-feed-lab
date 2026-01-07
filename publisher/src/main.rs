use std::io::Write;
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:9000";
    let mut stream = TcpStream::connect(addr)?;
    println!("connected to {addr}");

    for i in 0..10000u64 {
        let now = SystemTime::now();
        let sent_nanos: u64 = now
            .duration_since(UNIX_EPOCH)
            .expect("Time gone backwards")
            .as_nanos() as u64;

        let seq: u64 = i;

        let msg = format!("{sent_nanos},{seq},BTC-USD,11111.00,1.112\n");
        stream.write_all(msg.as_bytes())?;
    }

    Ok(())
}
