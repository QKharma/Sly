use std::sync::Arc;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::commands::command::*;

pub async fn ping(msg: Box<MessageCreate>, http: Arc<HttpClient>) -> CommandResult<()> {
  http
    .create_message(msg.channel_id)
    .content("Pong!")?
    .exec()
    .await?;

  Ok(())
}
