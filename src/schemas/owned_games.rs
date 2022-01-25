use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OwnedGamesResponse {
  pub response: OwnedGameList,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OwnedGameList {
  pub game_count: usize,
  pub games: Vec<OwnedGame>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OwnedGame {
  pub appid: usize,
  pub name: String,
  pub playtime_forever: usize,
  pub img_icon_url: String,
  pub img_logo_url: String,
}
