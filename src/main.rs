use std::{io::Result, fs::File};
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<()>{
    ingest_masscan_data().await?;
    Ok(())
}

// let's presume that we already have json scan data from masscan

async fn ingest_masscan_data() -> Result<Vec<MinecraftServer>> {
    let scan = File::open("scan.json")?;
    let scanned_servers: Vec<PingedServer> = serde_json::from_reader(scan)?;
    println!("Scanned servers: {:?}", scanned_servers);

    return Ok(Vec::new());
}

struct MinecraftServer;

#[derive(Debug, Deserialize)]
struct PingedServer {
    ip: String,
    timestamp: String // This might become a problem in 2031 lol
}