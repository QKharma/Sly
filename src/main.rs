mod bot;
mod data;
mod commands;
use crate::bot::*;

fn main() {
  Sly::new().expect("could not start bot");
}
