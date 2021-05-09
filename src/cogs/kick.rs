use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};


#[command]
#[num_args(3)]
#[only_in(guilds)]
#[required_permissions("KICK_MEMBERS")]
pub async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{

	match (&args).single::<id::UserId>() {
		Err(_) =>
			{
				msg.channel_id
					.say(&ctx, "Not a valid Member ID.")
					.await?;
			}

		Ok(mem) => match (&args).single::<Strings>()
			{
				Err(_) =>
					{
						msg.channel_id
							.say(&ctx, "You need a valid reason for kicking.")
							.await?;
					}
				Ok(res) => match msg.guild_id
					{
						None =>
							{
								msg.channel_id
									.say(&ctx, "You need a valid server to kick a user.")
									.await?;
							}
						Some(g) =>
							{
								let member = g.member(&ctx, mem)
									.kick_with_reason(&ctx, 0, &res)
									.await?;
								//member.ban_with_reason(&ctx, 0, &res)
								//	.await.unwrap();
								msg.channel_id
									.say(&ctx, format!("Successfully Kicked user {}, for {}", member.user.name, &res))
									.await?;
							}
					}
			}

	}

	Ok(())
}