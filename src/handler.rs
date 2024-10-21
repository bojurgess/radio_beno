use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("Connected to gateway as {}", data_about_bot.user.tag())
    }
}
