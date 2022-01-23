use serde_json::json;
use std::{
  error::Error,
  fs::{self, File},
  io::Write,
  path::Path,
  sync::Arc,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::data::bindings::*;

pub async fn bind(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let args: Vec<&str> = msg.content.split(" ").collect();
  if args.len() < 1 {
    http
      .create_message(msg.channel_id)
      .content("Missing argument: Steam ID")?
      .exec()
      .await?;
    return Ok(());
  }
  let steam_id = args[1];
  let discord_id = msg.author.id;

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

  Ok(())
}
