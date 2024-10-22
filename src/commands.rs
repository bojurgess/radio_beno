pub struct Data {
    pub http_client: reqwest::Client,
}

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;

pub mod voice;

use poise::ReplyHandle;
use voice::*;

pub type Commands = Vec<poise::Command<Data, Error>>;

pub fn voice_commands() -> Vec<poise::Command<Data, Error>> {
    vec![join(), play()]
}

pub fn check_msg(result: Result<ReplyHandle<'_>, serenity::Error>) {
    if let Err(why) = result {
        eprintln!("Command error: {:?}", why);
    }
}
