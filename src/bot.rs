use futures::stream::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;
use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::{Cluster, ShardScheme}, Event};
use twilight_http::Client;
use twilight_model::gateway::Intents;

use crate::commands::{link_steam::LinkSteam, ping::ping, unlink_steam::unbind};

pub const BOT_PREFIX: char = '!';

pub struct Sly;

impl Sly {
  #[tokio::main]
  pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("SLY")?;
    let scheme = ShardScheme::Auto;
  
    let (cluster, mut events) = Cluster::builder(token.to_owned(), Intents::GUILD_MESSAGES)
      .shard_scheme(scheme)
      .build()
      .await?;

    let cluster = Arc::new(cluster);

    tokio::spawn(async move {
      cluster.up().await;
    });

    let http = Arc::new(Client::new(token));

//    let application_id = {
//      let response = http.current_user_application().exec().await?;
//      
//      response.model().await?.id
//      };

//    let ping = http
//      .interaction(application_id)
//      .create_global_command()
//      .chat_input("ping", "pong")?
//      .exec();
//
//    println!("{:?}", ping.await?);

    while let Some((shard_id, event)) = events.next().await {
      tokio::spawn(handle_event(shard_id, event, Arc::clone(&http)));
    }

    Ok(())
  }
}

async fn handle_event(
  shard_id: u64,
  event: Event,
  http: Arc<Client>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
  lazy_static! {
    static ref BOT_PREFIX_CHECK: Regex = Regex::new(&format!(r"{}.+", BOT_PREFIX)).unwrap();
    static ref COMMAND: Regex = Regex::new(r"^.(\S*)").unwrap();
  }
  match event {
    Event::MessageCreate(msg) if BOT_PREFIX_CHECK.is_match(&msg.content) => {
      let command = COMMAND
        .captures(&msg.content)
        .unwrap()
        .get(1)
        .map_or("", |m| m.as_str());
      println!("{:?}", command);
      if command == "ping" {
        ping(msg, http).await?;
      } else if command == "bind" {
        LinkSteam::exec(msg, http).await?;
      } else if command == "unbind" {
        unbind(msg, http).await?;
      }
    }

    Event::ShardConnected(_) => {
      println!("Connected on shard {}", shard_id);
    }
    _ => {}
  }

  Ok(())
}
