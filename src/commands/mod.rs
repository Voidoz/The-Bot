use deno_core::error::AnyError;
use serenity::builder::{CreateApplicationCommand};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction};
use serenity::async_trait;

#[async_trait]
pub trait BotCommand {
    async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<(), AnyError>;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

mod ping;
pub mod run;
mod update;

pub use ping::Ping;
pub use run::Run;
pub use update::Update;