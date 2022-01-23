use futures::stream::StreamExt;
use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{
  cluster::{Cluster, ShardScheme},
  Event,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::Intents;

use lazy_static::lazy_static;
use regex::Regex;

mod data;

mod commands;
use crate::commands::create_binding::*;
use crate::commands::ping::*;

const PREFIX: char = '!';

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
  let token = env::var("SLY").expect("token not found");

  let scheme = ShardScheme::Auto;

  let (cluster, mut events) = Cluster::builder(token.to_owned(), Intents::GUILD_MESSAGES)
    .shard_scheme(scheme)
    .build()
    .await?;
  let cluster = Arc::new(cluster);

  let cluster_spawn = Arc::clone(&cluster);

  tokio::spawn(async move {
    cluster_spawn.up().await;
  });

  let http = Arc::new(HttpClient::new(token));

  let cache = InMemoryCache::builder()
    .resource_types(ResourceType::MESSAGE)
    .build();

  while let Some((shard_id, event)) = events.next().await {
    cache.update(&event);

    tokio::spawn(handle_event(shard_id, event, Arc::clone(&http)));
  }

  Ok(())
}

async fn handle_event(
  shard_id: u64,
  event: Event,
  http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  lazy_static! {
    static ref PREFIX_CHECK: Regex = Regex::new(&format!(r"{}.+", PREFIX)).unwrap();
  }
  match event {
    Event::MessageCreate(msg) if PREFIX_CHECK.is_match(&msg.content) => {
      if msg.content.contains("ping") {
        ping(msg, http).await?;
      } else if msg.content.contains("bind") {
        create_binding(msg, http).await?;
      }
    }
    Event::ShardConnected(_) => {
      println!("Connected on shard {}", shard_id);
    }
    _ => {}
  }

  Ok(())
}