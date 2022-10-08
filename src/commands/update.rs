use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::{ApplicationCommandInteraction};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::async_trait;
use serenity::Result;
use crate::BotCommand;
use crate::helpers::IS_DEBUG;

pub struct Update;

#[async_trait]
impl BotCommand for Update {
    async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<()> {
        if cmd.user.id == ctx.http.get_current_application_info().await.unwrap().owner.id {
            cmd
                .create_interaction_response(&ctx.http, |response|
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message|
                            message.content("Initiated update process".to_string())
                        ),
                )
                .await
                .unwrap();

            std::process::Command::new("git")
                .arg("pull")
                .status()
                .unwrap();

            ctx.shard.shutdown_clean();

            let mut update_command = "cargo run --package thebot --bin thebot".to_owned();

            if !IS_DEBUG { update_command.push_str(" --release") }

            std::process::Command::new("cmd")
                .args(["/C", "start", "cmd", "/c", update_command.as_str()])
                .spawn()
                .unwrap();

            std::process::exit(0);
        } else {
            cmd
                .create_interaction_response(&ctx.http, |response|
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message|
                            message.content("Only my owner can run this command".to_string())
                        ),
                ).await
        }
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("update").description("A command to update live bot")
    }
}