use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SteamBinding {
  pub discord_id: Id<UserMarker>,
  pub steam_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bindings {
  pub steam_bindings: Vec<SteamBinding>,
}
