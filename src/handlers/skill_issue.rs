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

pub struct SkillIssue;

#[async_trait]
impl BotHandler for SkillIssue {
    async fn run(ctx: &Context, message: &Message) -> bool {
        match search_string(&message.content, &"skill issue".to_string(),  false) {
            Some(_) => {
                message.reply(&ctx, "Seems to me like your comedy game suffers from a skill issue too.").await.unwrap();
                true
            },
            _ => false
        }
    }
}