use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
pub async fn addModRole(ctx: &Context, msg: &Message) -> CommandResult {

	if msg == " "
	{

	}

	Ok(())
}