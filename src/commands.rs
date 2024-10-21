pub struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub mod voice;

use voice::join;

pub type Commands = Vec<poise::Command<Data, Error>>;

// put voice commands in global array variable
pub fn voice_commands() -> Vec<poise::Command<Data, Error>> {
    vec![join()]
}
