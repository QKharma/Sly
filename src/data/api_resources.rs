use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerResponse {
  pub response: Players,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Players {
  pub players: Vec<SteamUserSummary>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SteamUserSummary {
  pub steamid: String,
  pub personaname: String,
  pub avatar: String,
}
