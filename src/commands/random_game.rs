use rand::Rng;
use std::{env, error::Error, fs, sync::Arc};
use twilight_embed_builder::{EmbedBuilder, ImageSource};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::schemas::{bindings::Bindings, owned_games::OwnedGamesResponse};

pub async fn random_game(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let steam_api_key = env::var("SLY_STEAM").expect("steam api key not found");

  let content = fs::read_to_string("bindings.json")?;
  let values: Bindings = serde_json::from_str(&content)?;
  let steam_binding = values
    .steam_bindings
    .iter()
    .filter(|m| m.discord_id == msg.author.id)
    .next();

  match steam_binding {
    Some(binding) => {
      let steam_id = &binding.steam_id;
      let request_url = format!(
        "https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/?key={key}&steamid={steam_id}&include_appinfo=1&include_played_free_games=1&include_free_sub=1",
        steam_id = steam_id,
        key = steam_api_key
      );
      let response = reqwest::get(&request_url).await?;
      let owned_games_response: OwnedGamesResponse = response.json().await?;
      let owned_games_list = owned_games_response.response;
      let owned_games = owned_games_list.games;
      let games_count = owned_games_list.game_count;

      let random_index = rand::thread_rng().gen_range(0..games_count - 1);

      let random_game = &owned_games[random_index];

      let embed = EmbedBuilder::new()
        .thumbnail(ImageSource::url(&format!(
          "https://media.steampowered.com/steamcommunity/public/images/apps/{}/{}",
          random_game.appid, random_game.img_icon_url
        ))?)
        .title(&format!("{}", random_game.name))
        .color(0x28de98)
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
