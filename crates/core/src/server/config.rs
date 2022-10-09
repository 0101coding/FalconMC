use std::net::IpAddr;
use std::str::FromStr;

use confy::ConfyError;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use tokio::net::ToSocketAddrs;
use tracing::metadata::LevelFilter;

use crate::player::data::{LookAngles, Position};

static INSTANCE: OnceCell<FalconConfig> = OnceCell::new();

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FalconConfig {
    connection: ConnectionSettings,
    players: PlayerSettings,
    server: ServerSettings,
    versions: VersionSettings,
}

impl FalconConfig {
    // 47, 107, 108, 109, 110, 210, 315, 316, 335, 338, 340, --> for future update
    pub const ALLOWED_VERSIONS: [u32; 13] = [393, 401, 404, 477, 480, 485, 490, 498, 573, 575, 578, 735, 736];

    pub fn global() -> &'static FalconConfig { INSTANCE.get().expect("FalconConfig is not initialized!!") }

    pub fn init_config(name: &str) -> Result<(), ConfyError> {
        INSTANCE.set(confy::load_path(name)?).unwrap();
        Ok(())
    }

    pub fn server_port(&self) -> u16 { self.connection.server_port }

    pub fn server_ip(&self) -> IpAddr { self.connection.server_ip }

    pub fn server_socket_addrs(&self) -> impl ToSocketAddrs + '_ { (self.server_ip(), self.server_port()) }

    pub fn max_players(&self) -> i32 { self.server.max_players }

    pub fn description(&self) -> &str { &self.server.description }

    pub fn world_file(&self) -> Option<&str> { self.server.world.as_deref() }

    pub fn tracing_level(&self) -> LevelFilter { self.server.tracing_level }

    pub fn allow_flight(&self) -> bool { self.players.allow_flight }

    pub fn max_view_distance(&self) -> u8 { self.players.max_view_distance }

    pub fn spawn_pos(&self) -> Position { self.players.spawn_position }

    pub fn spawn_look(&self) -> LookAngles { self.players.spawn_look }

    pub fn excluded_versions(&self) -> &Vec<u32> { &self.versions.excluded }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionSettings {
    server_ip: IpAddr,
    server_port: u16,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        ConnectionSettings {
            server_port: 30000,
            server_ip: IpAddr::from_str("0.0.0.0").unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSettings {
    allow_flight: bool,
    max_view_distance: u8,
    spawn_position: Position,
    spawn_look: LookAngles,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings {
            allow_flight: false,
            max_view_distance: 10,
            spawn_position: Default::default(),
            spawn_look: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerSettings {
    #[serde(with = "tracing_serde")]
    tracing_level: LevelFilter,
    max_players: i32,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    world: Option<String>,
}

impl Default for ServerSettings {
    fn default() -> Self {
        ServerSettings {
            tracing_level: LevelFilter::INFO,
            max_players: -1,
            description: String::from("§eFalcon server§r§b!!!"),
            world: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct VersionSettings {
    excluded: Vec<u32>,
}

mod tracing_serde {
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serializer};
    use tracing::metadata::LevelFilter;

    pub fn serialize<S: Serializer>(level: &LevelFilter, serializer: S) -> Result<S::Ok, S::Error> {
        match *level {
            LevelFilter::OFF => serializer.serialize_str("off"),
            LevelFilter::ERROR => serializer.serialize_str("error"),
            LevelFilter::WARN => serializer.serialize_str("warn"),
            LevelFilter::INFO => serializer.serialize_str("info"),
            LevelFilter::DEBUG => serializer.serialize_str("debug"),
            LevelFilter::TRACE => serializer.serialize_str("trace"),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<LevelFilter, D::Error> {
        let input: &'de str = <&'de str>::deserialize(deserializer)?;
        match input.trim().to_lowercase().as_str() {
            "off" => Ok(LevelFilter::OFF),
            "error" => Ok(LevelFilter::ERROR),
            "warn" => Ok(LevelFilter::WARN),
            "info" => Ok(LevelFilter::INFO),
            "debug" => Ok(LevelFilter::DEBUG),
            "trace" => Ok(LevelFilter::TRACE),
            _ => Err(Error::unknown_variant(input, &["off", "error", "warn", "info", "debug", "trace"])),
        }
    }
}
