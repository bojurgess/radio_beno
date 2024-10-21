pub struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub mod join;
pub mod test;

pub use join::join;
pub use test::test;
