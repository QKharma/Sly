use std::{
  error::Error,
  fs::{self},
  sync::Arc,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::schemas::bindings::Bindings;

pub async fn unlink(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  let discord_id = msg.author.id;
  let content = fs::read_to_string("bindings.json")?;
  let mut values: Bindings = serde_json::from_str(&content)?;
  let mut removed = false;
  values.steam_bindings.retain(|binding| {
    if binding.discord_id == discord_id {
      removed = true;
      false
    } else {
      true
    }
  });

  fs::write("bindings.json", serde_json::to_string(&values)?)?;

  if removed == true {
    http
      .create_message(msg.channel_id)
      .content("Steam account unlinked")?
      .exec()
      .await?;
  } else {
    http
      .create_message(msg.channel_id)
      .content("No linked steam account found")?
      .exec()
      .await?;
  }

  Ok(())
}
