use std::{io::{Error, Result, ErrorKind}, fs::File, str::FromStr};
use async_minecraft_ping::{connect, StatusResponse, ServerDescription, ServerPlayers};
use serde::{Deserialize};
use uuid::{Uuid};

#[tokio::main]
async fn main() -> Result<()>{
    ingest_masscan_data().await?;
    Ok(())
}

// Let's presume that we already have json scan data from masscan

async fn ingest_masscan_data() -> Result<Vec<MinecraftServer>> {
    let scan = File::open("scan.json")?;
    let scanned_servers: Vec<PingedServer> = serde_json::from_reader(scan)?;
    


    return Ok(Vec::new());
}

async fn ping_server(target: PingedServer) -> Result<MinecraftServer> {
    if let Ok(connection) = connect(target.ip.clone()).await {// TODO: Tweak connection config
        if let Ok(ping) = connection.status().await {
            return Ok(MinecraftServer::from_status(ping.status, target.ip));
        }
    }
    return Err(Error::new(ErrorKind::NotFound, format!("A server could not be reached")))
}
#[derive(Debug)]
struct MinecraftServer {
    ip: String,
    version: String,
    motd: String,
    max_players: u32,
    players: Vec<Player>,
    /* 
        Future ideas: 
            - enforces-secure-profile
            - Seperate version and server type
    */

}

impl MinecraftServer {
    fn from_status(status: StatusResponse, ip: String) -> MinecraftServer {
        let mut description;
        match status.description {
            ServerDescription::Plain(text) => description = text,
            ServerDescription::Object { text } => description = text,
        }
        return MinecraftServer { 
            ip: ip,
            version: status.version.name, 
            motd: description, 
            max_players: status.players.max, 
            players: Player::from_server_players(status.players), 
        }
    }

}

#[derive(Debug)]
struct Player {
    username: String,
    uuid: Uuid
}

impl Player {
    fn from_server_players(server_players: ServerPlayers) -> Vec<Player> {
        let mut players: Vec<Player> = Vec::new();
        if let Some(server_player_vec) = server_players.sample {
            for player in server_player_vec.iter() {
                if let Ok(uuid) = Uuid::from_str(player.id.as_str()) {
                    players.push(Player::new(player.name.clone(), uuid))
                }
            }
        }
        return players
    }

    fn new(username: String, uuid: Uuid) -> Player {
        return Player { username: username, uuid: uuid }
    }
}

#[derive(Debug, Deserialize)]
struct PingedServer {
    ip: String,
    timestamp: String // This might become a problem in 2031 lol
}