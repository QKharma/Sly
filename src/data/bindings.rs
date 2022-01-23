use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SteamBinding {
  pub discord_id: isize,
  pub steam_id: isize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bindings{
  pub steam_bindings: Vec<SteamBinding>
}