use serenity::all::{ButtonStyle, ReactionType};
use songbird::input::AuxMetadata;

use crate::Context;

struct UnwrappedMetadata {
    title: String,
    artist: String,
    source_url: String,
    thumbnail: String,
    duration: chrono::Duration,
}

pub struct NowPlaying<'a> {
    metadata: UnwrappedMetadata,
    context: Context<'a>,
}

impl<'a> NowPlaying<'a> {
    pub fn new(metadata: AuxMetadata, context: Context<'a>) -> Self {
        let metadata = UnwrappedMetadata {
            title: metadata.title.unwrap(),
            artist: metadata.artist.unwrap(),
            source_url: metadata.source_url.unwrap(),
            thumbnail: metadata.thumbnail.unwrap(),
            duration: chrono::Duration::from_std(metadata.duration.unwrap()).unwrap(),
        };

        Self { metadata, context }
    }

    pub fn create_response(&self) -> poise::CreateReply {
        let seconds = self.metadata.duration.num_seconds() % 60;
        let minutes = self.metadata.duration.num_minutes() % 60;
        let hours = self.metadata.duration.num_hours();
        let duration = if hours > 0 {
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        };

        let title = "Now Playing";
        let description = format!(
            ">>> [{}]({})\n**Artist:** `{}`\n**Duration**: `[{}]`",
            self.metadata.title, self.metadata.source_url, self.metadata.artist, duration,
        );

        let footer_text = format!(
            "Requested by: {} • Today at {}",
            self.context.author().tag(),
            chrono::Utc::now().format("%H:%M")
        );
        let footer_icon = self.context.author().face();

        let footer = serenity::builder::CreateEmbedFooter::new(footer_text).icon_url(footer_icon);

        let buttons = vec![
            serenity::builder::CreateButton::new("skip_prev")
                .style(ButtonStyle::Primary)
                .emoji(ReactionType::Unicode("⏮️".to_string())),
            serenity::builder::CreateButton::new("play_pause")
                .style(ButtonStyle::Secondary)
                .emoji(ReactionType::Unicode("⏯️".to_string())),
            serenity::builder::CreateButton::new("stop")
                .style(ButtonStyle::Danger)
                .emoji(ReactionType::Unicode("⏹️".to_string())),
            serenity::builder::CreateButton::new("skip_next")
                .style(ButtonStyle::Primary)
                .emoji(ReactionType::Unicode("⏭️".to_string())),
        ];

        poise::CreateReply::default()
            .embed(
                serenity::all::CreateEmbed::default()
                    .title(title)
                    .description(description)
                    .footer(footer)
                    .thumbnail(self.metadata.thumbnail.clone()),
            )
            .components(vec![serenity::builder::CreateActionRow::Buttons(buttons)])
    }
}
