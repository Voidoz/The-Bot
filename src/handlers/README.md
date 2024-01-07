# Boilerplate
```rust
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

pub struct Foo;

#[async_trait]
impl BotHandler for Foo {
    async fn run(ctx: &Context, message: &Message) -> bool {
        true
    }
}
```