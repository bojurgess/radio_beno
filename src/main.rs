mod handler;
use handler::Handler;

use poise::serenity_prelude as serenity;

struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello, world!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![test()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await;
    client.unwrap().start().await.unwrap();
}
