use std::collections::HashSet;
use deno_core::anyhow::{anyhow, Error};
use deno_core::error::AnyError;
use fancy_regex::Regex;
use serenity::{
    builder::{
        CreateApplicationCommand
    }
};
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::{ApplicationCommandInteraction, ResolvedTarget};
use serenity::async_trait;
use serenity::model::id::MessageId;
use serenity::model::prelude::command::CommandType;
use serenity::prelude::TypeMapKey;
use tokio::join;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use crate::commands::BotCommand;
use crate::js::ConsoleMessage;

pub const NAME: &str = "Run JS/TS";

pub struct RunData {
    message_ids: HashSet<MessageId>,
    kill_channel: (Sender<MessageId>, Receiver<MessageId>)
}

impl Default for RunData {
    fn default() -> Self {
        Self {
            message_ids: HashSet::new(),
            kill_channel: mpsc::channel::<MessageId>(32)
        }
    }
}

impl TypeMapKey for RunData {
    type Value = Self;
}

pub enum RunEvent {
    Start(MessageId),
    Kill(MessageId)
}

fn get_code_blocks(string: &str) -> Vec<String> {
    Regex::new(r"(?is)(?<=```[jt]s\n)(.*?)(?=```)").unwrap()
        .find_iter(string)
        .map(|val|
            val
                .unwrap()
                .as_str()
                .to_string()
        )
        .collect()
}

pub struct Run;

fn make_message(message: &str) -> String {
    let mut response = "Running code...".to_string();

    if !message.is_empty() {
        response.push_str(&format!("\n```\n{}```", message))
    }

    response
}

async fn respond(content: &str, ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<(), AnyError> {
    cmd
        .create_interaction_response(&ctx.http, |response|
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.content(content)
                )
        ).await?;

    Ok(())
}

#[async_trait]
impl BotCommand for Run {
    async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) -> Result<(), AnyError> {
        let target = cmd.data.target().ok_or(anyhow!("No command target found!"))?;

        match target {
            ResolvedTarget::Message(message) => {
                let code_blocks = get_code_blocks(message.content.as_str());

                if !code_blocks.is_empty() {
                    let mut index = 0_usize;

                    if let Some(code_block) = code_blocks.get(index) as Option<&String> {
                        let (tx, mut rx) = mpsc::channel::<ConsoleMessage>(32);

                        let code_block = code_block.to_string();
                        let runner = tokio::task::spawn(async move {
                            crate::js::run(&code_block, tx).await
                        });

                        let listener: JoinHandle<Result<(), Error>> = {
                            let ctx = ctx.clone();
                            let cmd = cmd.clone();

                            tokio::task::spawn(async move {
                                let mut code_string = String::new();

                                respond(&make_message(&code_string), &ctx, &cmd).await?;

                                while let Some(msg) = rx.recv().await {
                                    if let ConsoleMessage::Exit = msg {
                                        break;
                                    }

                                    cmd.edit_original_interaction_response(&ctx.http, |res| {
                                        match msg {
                                            ConsoleMessage::Info(info) => {
                                                println!("Info: {}", info);

                                                code_string.push_str(format!("[info]: {}\n", info).as_str());
                                            },
                                            ConsoleMessage::Warn(warn) => {
                                                println!("Warn: {}", warn);

                                                code_string.push_str(format!("[warn]: {}\n", warn).as_str());
                                            },
                                            ConsoleMessage::Error(error) => {
                                                println!("Error: {}", error);

                                                code_string.push_str(format!("[error]: {}\n", error).as_str());
                                            },
                                            _ => {},
                                        }
                                        res.content(make_message(&code_string))
                                    }).await?;
                                }

                                Ok(())
                            })
                        };

                        let runs = join!(runner, listener);

                        runs.0??;
                        runs.1??;
                    }
                } else {
                    respond("No code blocks found!", &ctx, &cmd).await?;
                }
            }
            _ => {
                respond("This command may only be used on messages!", &ctx, &cmd).await?;
            }
        };

        Ok(())
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .kind(CommandType::Message)
            .name(NAME)
    }
}