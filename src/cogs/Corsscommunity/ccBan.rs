use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};



//TODO: hook up sql cross community group hook up.
#[command]
#[num_args(3)]
#[only_in("guilds")]
#[required_permissions("BAN_MEMBERS")]
pub async fn ccBan(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{

	match (&args).single::<id::UserId>() {
		Err(_) =>
			{
				msg.channel_id
					.say(&ctx, "Not a valid Member ID for a cross community ban.")
					.await?;
			}

		Ok(mem) => match (&args).single::<Strings>()
		{
			Err(_) =>
				{
					msg.channel_id
						.say(&ctx, "You need a valid reason for a cross community banning.")
						.await?;
				}
			Ok(res) => match msg.guild_id
			{
				None =>
					{
						msg.channel_id
							.say(&ctx, "You need a valid server to a cross community ban a user.")
							.await?;
					}
				Some(g) =>
					{
						//ToDo: implement ccBans with sqlite calls.
						if false
						{
							let member = g.member(&ctx, mem)
								.ban_with_reason(&ctx, 0, &res)
								.await?;
							member.ban_with_reason(&ctx, 0, &res)
								.await?;
						}
						else {
							msg.channel_id
								.say(&ctx, format!("Did not ccBanned {}, command is a WIP", member.user.name))
								.await?;
						}
					}
			}
		}

	}

	Ok(())
}
