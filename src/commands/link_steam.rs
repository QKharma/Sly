use serde_json::json;
use std::{
  env,
  error::Error,
  fs::{self, File},
  io::Write,
  path::Path,
  sync::Arc,
};
use twilight_embed_builder::{EmbedBuilder, ImageSource};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::schemas::bindings::*;
use crate::schemas::steam_user::*;

pub async fn link(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let steam_api_key = env::var("SLY_STEAM")?;

  let args: Vec<&str> = msg.content.split(' ').collect();
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
  let response = reqwest::get(&request_url).await?;
  let player_response: PlayerResponse = response.json().await?;
  let players: Players = player_response.response;
  let account_data: Option<&SteamUserSummary> = players.players.first();
  match account_data {
    Some(_) => (),
    None => {
      http
        .create_message(msg.channel_id)
        .content("Steam account does not exist")?
        .exec()
        .await?;
      return Ok(());
    }
  }

  let account_data = account_data.unwrap();

  if !Path::new("bindings.json").exists() {
    let mut f = File::create("bindings.json")?;
    let empty_bindings = json!({
      "steam_bindings": []
    });
    f.write_all(serde_json::to_string(&empty_bindings)?.as_bytes())?;
  }
  let content = fs::read_to_string("bindings.json")?;
  let mut values: Bindings = serde_json::from_str(&content)?;
  for binding in values.steam_bindings.clone() {
    if binding.discord_id == discord_id {
      http
        .create_message(msg.channel_id)
        .content(&format!(
          "{discord_name} is already linked to a steam account",
          discord_name = (format!(
            "{}#{}",
            &msg.author.name,
            &msg.author.discriminator.to_string()
          ))
        ))?
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
    .title(&format!(
      "{discord_name} linked to steam user: {steam_name}",
      discord_name = (format!(
        "{}#{}",
        &msg.author.name,
        &msg.author.discriminator.to_string()
      )),
      steam_name = account_data.personaname
    ))
    .color(0x28de98)
    .build()?;

  http
    .create_message(msg.channel_id)
    .embeds(&[embed])?
    .exec()
    .await?;

  Ok(())
}
