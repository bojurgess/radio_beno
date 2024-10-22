use serenity::{
    all::{Context, CreateInteractionResponseFollowup, EventHandler, Interaction, Ready},
    async_trait,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, data_about_bot: Ready) {
        println!("Connected to gateway as {}", data_about_bot.user.tag())
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Component(interaction) => {
                let guild_id = interaction.guild_id.unwrap();

                let manager = songbird::get(&ctx)
                    .await
                    .expect("Songbird Voice client is not initialized")
                    .clone();

                if let Some(handler_lock) = manager.get(guild_id) {
                    let handler = handler_lock.lock().await;
                    match interaction.data.custom_id.as_str() {
                        "skip_prev" => {
                            // Handle skip_prev button
                        }
                        "skip_next" => {
                            // Handle skip_next button
                        }
                        "play_pause" => {
                            // Handle pause button
                            handler.current_channel()
                        }
                        _ => {}
                    }
                } else {
                    interaction
                        .create_followup(
                            &ctx.http,
                            CreateInteractionResponseFollowup::new()
                                .content("Not in a voice channel"),
                        )
                        .await
                        .unwrap();
                    return;
                }
            }
            _ => {}
        }
    }
}
