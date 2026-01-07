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

        //Split into 5 fields
        let mut parts = line.split(',');

        let sent_nanos = match parts.next().and_then(|s| s.parse::<u64>().ok()) {
            Some(v) => v,
            None => {
                eprintln!("bad line (sent_nanos): {line}");
                continue;
            }
        };

        let seq = match parts.next().and_then(|s| s.parse::<u64>().ok()) {
            Some(v) => v,
            None => {
                eprintln!("bad line (seq): {line}");
                continue;
            }
        };

        let symbol = match parts.next() {
            Some(v) if !v.is_empty() => v,
            _ => {
                eprintln!("bad line (symbol): {line}");
                continue;
            }
        };

        let price = match parts.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => {
                eprintln!("bad line (price): {line}");
                continue;
            }
        };

        let size = match parts.next().and_then(|s| s.parse::<f64>().ok()) {
            Some(v) => v,
            None => {
                eprintln!("bad line (size): {line}");
                continue;
            }
        };

        // detect an extra field (badly formatted data)
        if parts.next().is_some() {
            eprintln!("bad line (too many fields): {line}");
            continue;
        }

        println!(
            "tick seq={seq} symbol={symbol} price={price} size={size} sent_nanos={sent_nanos}"
        );
    }

    println!("connection closed");
    Ok(())
}
