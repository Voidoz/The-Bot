use serenity::{
    client::{
        Context
    },
    model::{
        channel::{
            Message
        }
    }
};
use crate::async_trait;
use crate::handlers::BotHandler;
use crate::helpers::search_string;

pub struct UrMum;

#[async_trait]
impl BotHandler for UrMum {
    async fn run(ctx: &Context, message: &Message) -> bool {
        let mut success = false;

        match search_string(&message.content, &"ur mum".to_string(), false) {
            Some(_) => success = true,
            _ => match search_string(&message.content, &"ur mum".to_string(), false) {
                Some(_) => success = true,
                _ => {}
            }
        }

        if success {
            message.reply(&ctx, "**Fun fact:** That's what I'm doing right now.").await.unwrap();
        }

        success
    }
}