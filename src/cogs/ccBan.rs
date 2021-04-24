use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

//TODO
#[command]
#[num_args(3)]
#[only_in("guilds")]
#[required_permissions("BAN_MEMBER")]
pub async fn ccBan(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{

	match (&args).single::<id::UserId>() {
		Err(_) =>
			{
				msg.channel_id
					.say(&ctx, "Not a vaild Member ID for ccban.")
					.await.unwrap();
			}

		Ok(mem) => match (&args).single::<Strings>()
		{
			Err(_) =>
				{
					msg.channel_id
						.say(&ctx, "You need a vaild reason for ccbanning.")
						.await.unwrap();
				}
			Ok(res) => match msg.guild_id
			{
				None =>
					{
						msg.channel_id
							.say(&ctx, "You need a vaild server to ccban a user.")
							.await.unwrap();
					}
				Some(g) =>
					{
						if (false)
						{
							let member = g.member(&ctx, mem)
								.ban_with_reason(&ctx, 0, &res)
								.await.unwrap();
							member.ban_with_reason(&ctx, 0, &res)
							.	await.unwrap();
						}
						msg.channel_id
							.say(&ctx, format!("Did not ccBanned {}, command is a WIP", member.user.name))
							.await.unwrap();
					}
			}
		}

	}

	Ok(())
}
