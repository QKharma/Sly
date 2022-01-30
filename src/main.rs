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

mod schemas;

mod commands;
use crate::commands::link_steam::link;
use crate::commands::ping::ping;
use crate::commands::random_game::random_game;
use crate::commands::random_wishlist_game::random_wishlist_game;
use crate::commands::unlink_steam::unlink;

const PREFIX: char = '!';

fn main() {
  if let Err(e) = run() {
    println!("{:?}", e);
  }
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
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

    let task = tokio::spawn(handle_event(shard_id, event, Arc::clone(&http)));
    match task.await? {
      Err(e) => println!("{}", e),
      _ => (),
    };
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
    static ref COMMAND: Regex = Regex::new(r"^.(\S*)").unwrap();
  }
  match event {
    Event::MessageCreate(msg) if PREFIX_CHECK.is_match(&msg.content) => {
      let command = COMMAND
        .captures(&msg.content)
        .unwrap()
        .get(1)
        .map_or("", |m| m.as_str());
      println!("{:?}", command);
      if command == "ping" {
        ping(msg, http).await?;
      } else if command == "link" {
        link(msg, http).await?;
      } else if command == "unlink" {
        unlink(msg, http).await?;
      } else if command == "rg" {
        random_game(msg, http).await?;
      } else if command == "rw" {
        random_wishlist_game(msg, http).await?;
      }
    }
    Event::ShardConnected(_) => {
      println!("Connected on shard {}", shard_id);
    }
    _ => {}
  }

  Ok(())
}
