mod bot;
mod commands;
mod data;
use crate::bot::*;

fn main() {
  match Sly::run() {
    Ok(_) => (),
    Err(e) => println!("startup failed: {:?}", e),
  }
}
