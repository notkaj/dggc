#![allow(incomplete_features)]
#![allow(dead_code)]
mod actualization;
mod app;
mod chat;
mod connection;
mod conversion;
mod crossterm;
mod data;
mod flairs;
mod history;
mod ui;
mod update;

use crate::crossterm::run;
use argh::FromArgs;
use std::{error::Error, time::Duration};

#[derive(Debug, FromArgs)]
#[argh(description = "App features")]
struct Cli {
    /// time in ms between two clicks
    #[argh(option, default = "250")]
    tick_rate: u64,
    #[argh(option, default = "true", description = "use enhanced graphics?")]
    enhanced_graphics: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //println!("Connecting to dgg chat");
    //connection::connect().await;
    let cli: Cli = argh::from_env();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    run(tick_rate, cli.enhanced_graphics).await
    // Ok(())
}
