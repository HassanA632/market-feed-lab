use std::io::{BufRead, BufReader};
use std::net::TcpListener;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

fn unix_nanos_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time gone backwards")
        .as_nanos() as u64
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:9000";
    let listener = TcpListener::bind(addr)?;
    println!("consumer listening on {}", addr);

    let (stream, peer) = listener.accept()?;
    println!("connected: {}", peer);

    // start collection stats
    let start = Instant::now();
    let mut count: u64 = 0;
    let mut latencies_ns: Vec<u64> = Vec::new();
    let mut proc_latencies_ns: Vec<u64> = Vec::new();

    // read lines from stream
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        let line = line?;

        let t0 = Instant::now();

        //Split into 5 fields
        let mut parts = line.split(',');

        let sent_nanos = match parts.next().and_then(|s| s.parse::<u64>().ok()) {
            Some(v) => v,
            None => {
                eprintln!("bad line (sent_nanos): {line}");
                continue;
            }
        };

        // Compute latency and store in latencies_ns vector
        let recv_nanos = unix_nanos_now();
        let latency = recv_nanos.saturating_sub(sent_nanos);
        latencies_ns.push(latency);
        count += 1;

        let proc_ns = t0.elapsed().as_nanos() as u64;
        proc_latencies_ns.push(proc_ns);

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

    let elapsed = start.elapsed().as_secs_f64();
    let throughput = (count as f64) / elapsed.max(1e-9);

    latencies_ns.sort_unstable();
    proc_latencies_ns.sort_unstable();

    fn percentile(sorted: &[u64], p: f64) -> u64 {
        if sorted.is_empty() {
            return 0;
        }
        let n = sorted.len();
        let idx = ((p * (n as f64 - 1.0)).round() as usize).min(n - 1);
        sorted[idx]
    }

    // Percentiles (p50/p95/p99) show typical latency (p50) and tail latency spikes (p95/p99) which matter ALOT in low latency systems.
    let p50 = percentile(&latencies_ns, 0.50);
    let p95 = percentile(&latencies_ns, 0.95);
    let p99 = percentile(&latencies_ns, 0.99);

    let proc_p50 = percentile(&proc_latencies_ns, 0.50);
    let proc_p95 = percentile(&proc_latencies_ns, 0.95);
    let proc_p99 = percentile(&proc_latencies_ns, 0.99);

    //-------------STATS----------------
    println!("--- stats ---");
    println!("msgs: {count}");
    println!("elapsed: {elapsed:.3}s");
    println!("throughput: {:.0} msg/s", throughput);

    // Convert ns to microseconds for readability
    println!("latency p50: {:.2} µs", (p50 as f64) / 1_000.0);
    println!("latency p95: {:.2} µs", (p95 as f64) / 1_000.0);
    println!("latency p99: {:.2} µs", (p99 as f64) / 1_000.0);

    println!("PROC latency p50: {:.2} µs", (proc_p50 as f64) / 1_000.0);
    println!("PROC latency p95: {:.2} µs", (proc_p95 as f64) / 1_000.0);
    println!("PROC latency p99: {:.2} µs", (proc_p99 as f64) / 1_000.0);

    println!("connection closed");
    Ok(())
}
