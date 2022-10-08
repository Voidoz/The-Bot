use serenity::builder::{CreateApplicationCommand};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction};
use serenity::Result;
use serenity::async_trait;

#[async_trait]
pub trait BotCommand {
    async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()>;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

mod ping;
mod update;

pub use ping::Ping;
pub use update::Update;