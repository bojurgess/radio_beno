use crate::commands::{Context, Error};

#[poise::command(prefix_command)]
pub async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    if ctx.author().id != 374988401595908098 {
        ctx.reply("You are not authorized to use this command")
            .await?;
        return Ok(());
    }

    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}
