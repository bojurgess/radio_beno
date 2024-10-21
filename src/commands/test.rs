use super::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn test(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello, world!").await?;
    Ok(())
}
