use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

#[command]
#[only_in(guilds)]
#[required_permissions("ADMINISTRATOR")]
//ToDo: Add in a system which it allows admins to add role that enables user to use the bot without api permisions
pub async fn add_mod_role(ctx: &Context, msg: &Message) -> CommandResult
{

	if msg == " "
	{

	}

	Ok(())
}