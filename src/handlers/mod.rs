use serenity::async_trait;
use serenity::client::Context;
use serenity::model::channel::Message;

#[async_trait]
pub trait BotHandler {
    async fn run(ctx: &Context, message: &Message) -> bool;
}

mod dad;
pub use dad::Dad;

mod skill_issue;
pub use skill_issue::SkillIssue;