use std::{error::Error, fs::{File, self}, sync::Arc, io::Write, path::Path};
use serde_json::Value;
use twilight_http::Client as HttpClient;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::data::bindings::*;

pub async fn create_binding(
  msg: Box<MessageCreate>,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  if !Path::new("bindings.json").exists(){
    File::create("bindings.json").expect("creating bindings.json failed");
  }
  let content = fs::read_to_string("bindings.json")?;
  let values: Bindings = serde_json::from_str(&content).expect("fuck");
  println!("{:?}", values);
  for binding in values.steam_bindings{
    println!("{:?}", binding.discord_id)
  }
  Ok(())
}
