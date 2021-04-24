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
					.say(&ctx, "Not a vaild Member ID.")
					.await.unwrap();
			}

		Ok(mem) => match (&args).single::<Strings>()
			{
				Err(_) =>
					{
						msg.channel_id
							.say(&ctx, "You need a vaild reason for banning.")
							.await.unwrap();
					}
				Ok(res) => match msg.guild_id
					{
						None =>
							{
								msg.channel_id
									.say(&ctx, "You need a vaild server to ban a user.")
									.await.unwrap();
							}
						Some(g) =>
							{
								let member = g.member(&ctx, mem)
									.kick_with_reason(&ctx, 0, &res)
									.await.unwrap();
								//member.ban_with_reason(&ctx, 0, &res)
								//	.await.unwrap();
								msg.channel_id
									.say(&ctx, format!("Succesfully Kicked user {}, for {}", member.user.name, &res))
									.await.unwrap();
							}
					}
			}

	}

	Ok(())
}