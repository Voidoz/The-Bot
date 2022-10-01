use serenity::{
    builder::{
        CreateApplicationCommand
    },
    model::{
        prelude::{
            interaction::{
                application_command::{
                    CommandDataOption
                }
            }
        }
    }
};
use serenity::builder::CreateInteractionResponse;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::Result;
use serenity::async_trait;
use crate::commands::BotCommand;

pub struct Ping;

#[async_trait]
impl BotCommand for Ping {
    async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
        cmd
            .create_interaction_response(&ctx.http, |response|
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message|
                        message.content("Hey, I'm alive!".to_string())
                    ),
            ).await
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("ping").description("A ping command")
    }
}