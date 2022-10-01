use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

#[async_trait]
pub trait BotHandler {
    async fn run(ctx: &Context, message: &Message);
}

mod dad;
pub use dad::Dad;