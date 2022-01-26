use std::{error::Error, sync::Arc};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn ping(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  http
    .create_message(msg.channel_id)
    .content("Pong!")?
    .exec()
    .await?;

  Ok(())
}
