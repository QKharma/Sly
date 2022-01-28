use rand::Rng;
use std::{collections::HashMap, error::Error, fs, sync::Arc};
use twilight_embed_builder::{EmbedBuilder, ImageSource};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::schemas::{bindings::Bindings, wishlist_data::{WishlistGame, GameSub}};

pub async fn random_wishlist_game(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let content = fs::read_to_string("bindings.json")?;
  let values: Bindings = serde_json::from_str(&content)?;
  let steam_binding = values
    .steam_bindings
    .iter()
    .find(|m| m.discord_id == msg.author.id);

  match steam_binding {
    Some(binding) => {
      let steam_id = &binding.steam_id;
      let request_url = format!(
        "https://store.steampowered.com/wishlist/profiles/{steam_id}/wishlistdata/",
        steam_id = steam_id,
      );
      let response = reqwest::get(&request_url).await?;
      let games: HashMap<String, WishlistGame> = response.json().await?;
      let games: Vec<&WishlistGame> = games.values().collect();
      let games_count = games.len();

      let random_index = rand::thread_rng().gen_range(0..games_count - 1);

      let random_game = &games[random_index];

      let game_sub = random_game.subs.first().unwrap_or(&GameSub {discount_pct: 0, price: 0});

      let gametags = random_game.tags.clone();
      let gametags_str: Option<String> = gametags
        .into_iter()
        .reduce(|s1, s2| format!("{}, {}", s1, s2));

      let mut old_price = String::from("");
      let mut disclaimer = String::from("");
      
      if game_sub.discount_pct > 0 {
        old_price = format!("~~{}$~~ ",game_sub.price/(100-game_sub.discount_pct));
      }

      if random_game.r#type == "DLC" {
        disclaimer = String::from("**DLC prices currently not available**");
      }

      let description = format!(
        "Review score: {review_score}
        Tags: {tags}
        Discount: {discount}%
        Price: {old_price}{price:.2}${disclaimer}",
        review_score = random_game.review_score,
        tags = gametags_str.unwrap_or("no tags found".to_string()),
        discount = game_sub.discount_pct,
        price = game_sub.price/100,
        disclaimer = disclaimer,
        old_price = old_price
      );

      let embed = EmbedBuilder::new()
        .title(&format!("You should buy: {}", random_game.name))
        .image(ImageSource::url(&format!("{}", random_game.capsule))?)
        .color(0x28de98)
        .description(description)
        .build()?;

      http
        .create_message(msg.channel_id)
        .embeds(&[embed])?
        .exec()
        .await?;
    }
    None => {
      http
        .create_message(msg.channel_id)
        .content("No linked steam account found")?
        .exec()
        .await?;
    }
  }

  Ok(())
}
