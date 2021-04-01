use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

#[command]
#[only_in(guilds)]
#[allow_roles("mods")]
pub async fn ban(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

	if let Some(member) = &msg.member {

		for role in &member.roles {
			if role.to_role_cached(&ctx.cache).await.map_or(false, |r| r.has_permission(Permissions::BAN_MEMBERS)) {

				guild.ban_with_reason(user, 4, reason);

				return Ok(());
			}
		}
	}

	msg.channel_id.say(&ctx.http, "No, you are not.").await?;

	Ok(())
}
