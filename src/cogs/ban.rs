use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

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
					.say(&ctx, "Not a vaild Member ID.")
					.await?;
			}

		Ok(mem) => match (&args).single::<Strings>()
		{
			Err(_) =>
				{
					msg.channel_id
						.say(&ctx, "You need a vaild reason for banning.")
						.await?;
				}
			Ok(res) => match msg.guild_id
			{
				None =>
					{
						msg.channel_id
							.say(&ctx, "You need a vaild server to ban a user.")
							.await?;
					}
				Some(g) =>
					{
						let member = g.member(&ctx, mem)
							.ban_with_reason(&ctx, 0, &res)
							.await?;
						//member.ban_with_reason(&ctx, 0, &res)
						//	.await.unwrap();
						msg.channel_id
							.say(&ctx, format!("Succesfully Banned {}, for: {}", member.user.name, &res))
							.await?;
					}
			}
		}

	}

	Ok(())
}
