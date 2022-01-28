use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WishlistGame {
  pub name: String,
  pub capsule: String,
  pub review_score: usize,
  pub tags: Vec<String>,
  pub subs: Vec<GameSub>,
  pub r#type: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameSub {
  pub price: usize,
  pub discount_pct: usize,
}
