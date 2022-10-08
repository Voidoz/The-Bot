extern crate core;

mod commands;
mod database;
mod handlers;
mod helpers;

use serenity::{
    async_trait,
    model::{
        application::{
            command::Command,
            interaction::{
                Interaction,
                InteractionResponseType
            }
        },
        channel::Message,
        gateway::Ready
    },
    prelude::*
};
use diesel::{SqliteConnection};
use diesel::r2d2::{ConnectionManager, Pool};

use crate::commands::BotCommand;
use crate::database::{get_config, get_connection_pool};
use crate::handlers::BotHandler;

pub struct PoolData;
impl TypeMapKey for PoolData {
    type Value = Pool<ConnectionManager<SqliteConnection>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, message: Message) {
        if message.author.bot { return; }

        let data = ctx.data.read().await;

        let con = data.get::<PoolData>().unwrap();

        let dev_channel = get_config(&mut con.get().unwrap()).unwrap().dev_channel;

        if helpers::should_handle(&message.channel_id, &dev_channel) {
            if handlers::Dad::run(&ctx, &message).await { return; }
            else if handlers::SkillIssue::run(&ctx, &message).await { return; }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_command = Command::set_global_application_commands(&ctx.http, |cmds| {
            cmds
                .create_application_command(|cmd| commands::Ping::register(cmd))
                .create_application_command(|cmd| commands::Update::register(cmd))
        })
            .await;

        println!("I created the following global slash command: {:#?}", guild_command);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let data = ctx.data.read().await;

        let con = data.get::<PoolData>().unwrap();

        let dev_channel = get_config(&mut con.get().unwrap()).unwrap().dev_channel;

        match interaction {
            Interaction::ApplicationCommand(command) =>
                if helpers::should_handle(&command.channel_id, &dev_channel) {
                    println!("Received command interaction: {:#?}", dev_channel);

                    if let Err(why) = match command.data.name.as_str() {
                        "ping" => commands::Ping::run(&ctx, &command),
                        "update" => commands::Update::run(&ctx, &command),
                        _ => Box::pin(command
                            .create_interaction_response(&ctx.http, |response|
                                response
                                    .kind(InteractionResponseType::ChannelMessageWithSource)
                                    .interaction_response_data(|message|
                                        message.content("not implemented :(".to_string())
                                    )
                            ))
                    }.await {
                        println!("Cannot respond to slash command: {}", why);
                    }
                },
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();

    let con = &mut pool.get().unwrap();

    let token = get_config(con).unwrap().token;

    // Build our client.
    let mut client = Client::builder(
        token,
        GatewayIntents::MESSAGE_CONTENT |
            GatewayIntents::GUILD_MESSAGES,
    )
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<PoolData>(pool);
    }

    client.start().await.unwrap();

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}