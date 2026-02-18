mod bbn_date_range;
mod command;
mod config;
mod config_repository;
mod credentials;
mod date_like;

pub use bbn_date_range::bbn_date_range;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let command = <command::Command as clap::Parser>::parse();
    command.handle().await
}
