use serde_json::json;
use twilight_embed_builder::{EmbedBuilder, ImageSource};
use std::{
  error::Error,
  fs::{self, File},
  io::Write,
  path::Path,
  sync::Arc, env,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::data::bindings::*;
use crate::data::api_resources::*;

pub async fn bind(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let steam_api_key = env::var("SLY_STEAM").expect("steam api key not found");

  let args: Vec<&str> = msg.content.split(" ").collect();
  if args.len() < 2 {
    http
      .create_message(msg.channel_id)
      .content("Missing argument: Steam ID")?
      .exec()
      .await?;
    return Ok(());
  }
  let steam_id = args[1];
  let discord_id = msg.author.id;

  let request_url = format!(
    "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={key}&steamids={steam_id}",
    steam_id = steam_id,
    key = steam_api_key
  );
  let response = reqwest::get(&request_url).await.expect("trash");
  let player_response: PlayerResponse = response.json().await.expect("trash");
  let players: Players = player_response.response;
  let account_data: Option<&SteamUserSummary> = players.players.first();
  match account_data{
    Some(_) => (),
    None => {
      http
        .create_message(msg.channel_id)
        .content("Steam account does not exist")?
        .exec()
        .await?;
      return Ok(());
    },
  }

  let account_data = account_data.unwrap();

  if !Path::new("bindings.json").exists() {
    let mut f = File::create("bindings.json")?;
    let empty_bindings = json!({
      "steam_bindings": []
    });
    f.write(serde_json::to_string(&empty_bindings)?.as_bytes())?;
  }
  let content = fs::read_to_string("bindings.json")?;
  let mut values: Bindings = serde_json::from_str(&content)?;
  for binding in values.steam_bindings.clone() {
    if binding.discord_id == discord_id {
      http
        .create_message(msg.channel_id)
        .content("Account is already bound")?
        .exec()
        .await?;
      return Ok(());
    }
  }
  let new_entry = SteamBinding {
    discord_id,
    steam_id: steam_id.to_string(),
  };
  values.steam_bindings.push(new_entry);
  fs::write("bindings.json", serde_json::to_string(&values)?)?;

  let embed = EmbedBuilder::new()
    .thumbnail(ImageSource::url(&account_data.avatar)?)
    .title(&format!("Account bound to steam user: {}", account_data.personaname))
    .color(0x28de98)
    .build()?;
  
  http
    .create_message(msg.channel_id)
    .embeds(&[embed])?
    .exec()
    .await?;

  Ok(())
}
