use songbird::input::{Compose, YoutubeDl};

use crate::{
    check_msg,
    commands::{Context, Error},
    components,
};

#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "The URL of the song to play"] url: String,
) -> Result<(), Error> {
    let (guild_id, _channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);
        (guild.id, channel_id)
    };

    let should_search = !url.starts_with("http");

    let http_client = ctx.data().http_client.clone();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client is not initialized")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let mut src = if should_search {
            YoutubeDl::new_search(http_client, url)
        } else {
            YoutubeDl::new(http_client, url)
        };
        let _handle = handler.play_input(src.clone().into());
        let metadata = src.aux_metadata().await?;
        tokio::spawn(async move {
            loop {
                match src.aux_metadata().await {
                    Ok(info) => println!("{:?}", info),
                    Err(err) => eprintln!("Error getting info: {:?}", err),
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        let reply = components::NowPlaying::new(metadata, ctx).create_response();

        check_msg(poise::send_reply(ctx, reply).await);
    } else {
        check_msg(ctx.reply("Not in a voice channel").await);
    }

    Ok(())
}
