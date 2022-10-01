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

pub struct Dad;

#[async_trait]
impl BotHandler for Dad {
    async fn run(ctx: &Context, message: &Message) {
        // Range of u8 is 0-255
        let random: u8 = rand::thread_rng().gen();

        // Check if number is less than 26 (25.5 is 10% of 255)
        if random < 26 {
            let content = &message.content;
            let lower = &content.to_lowercase();

            let found_string =
                if      lower.starts_with("im ") { Some("im ") }
                else if lower.contains(" im ") { Some(" im ") }
                else if lower.ends_with(" im") { Some(" im") }

                else if lower.starts_with("i'm ") { Some("i'm ") }
                else if lower.contains(" i'm ") { Some(" i'm ") }
                else if lower.ends_with(" i'm") { Some(" i'm") }

                else if lower.starts_with("i am ") { Some("i am ") }
                else if lower.contains(" i am ") { Some(" i am ") }
                else if lower.ends_with("i am ") { Some(" i am") }
                else { None };

            match found_string {
                Some(found_string) => {
                    let index = lower.find(found_string).unwrap();

                    let mut msg = "Hi ".to_owned();

                    msg.push_str(&content[index + found_string.len()..content.len()].trim());

                    msg.push_str(", I'm Dad.");

                    message.reply(&ctx, &msg).await.unwrap();
                },
                _ => {}
            }
        }
    }
}