use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

#[command]
#[only_in(guilds)]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult
{
	msg.channel_id.say(&ctx.http, "pong").await?;

	Ok(())
}