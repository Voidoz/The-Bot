use rand::Rng;
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

pub struct Dad;

#[async_trait]
impl BotHandler for Dad {
    async fn run(ctx: &Context, message: &Message) -> bool {
        // Range of u8 is 0-255
        let random: u8 = rand::thread_rng().gen();

        // Check if number is less than 26 (25.5 is 10% of 255)
        if random < 26 {
            let content = &message.content;
            let lower = &content.to_lowercase();

            let mut found_string: Option<String> = None;

            match search_string(lower, &"im".to_string(), true) {
                Some(str) => found_string = Some(str),
                _ => {
                    match search_string(lower, &"i'm".to_string(), true) {
                        Some(str) => found_string = Some(str),
                        _ => {
                            match search_string(lower, &"i am".to_string(), true) {
                                Some(str) => found_string = Some(str),
                                _ => {}
                            }
                        }
                    }
                }
            }

            match found_string {
                Some(found_string) => {
                    let index = lower.find(&found_string).unwrap();

                    let mut msg = "Hi ".to_owned();

                    msg.push_str(&content[index + found_string.len()..content.len()].trim());

                    msg.push_str(", I'm Dad.");

                    message.reply(&ctx, &msg).await.unwrap();

                    true
                },
                _ => false
            }
        } else { false }
    }
}