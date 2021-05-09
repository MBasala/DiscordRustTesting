use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

//ToDo: ban override with special perms to ban a user.
//Stub
#[command]
#[num_args(3)]
#[only_in(guilds)]
#[required_permissions("ADMIN")]
pub async fn ccBanOverride(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{

	match (&args).single::<id::UserId>() {
		Err(_) =>
			{
				msg.channel_id
					.say(&ctx, "Not a valid Member ID.")
					.await.unwrap();
			}

		Ok(mem) => match (&args).single::<Strings>()
		{
			Err(_) =>
				{
					msg.channel_id
						.say(&ctx, "You need a valid reason for banning.")
						.await.unwrap();
				}
			Ok(res) => match msg.guild_id
			{
				None =>
					{
						msg.channel_id
							.say(&ctx, "You need a valid server to cross community ban a user.")
							.await.unwrap();
					}
				Some(g) =>
					{
						if false
						{
							let member = g.member(&ctx, mem)
								.kick_with_reason(&ctx, 0, &res)
								.await.unwrap();
							//member.ban_with_reason(&ctx, 0, &res)
							//	.await.unwrap();
							msg.channel_id
								.say(&ctx, format!("Succesfully commited an override cross community ban for user {}, for {} reasons", member.user.name, &res))
								.await.unwrap();
						}
						else
						{
							msg.channel_id
								.say(&ctx, format!("Command is not implemented"))
								.await.unwrap();
						}
					}
			}
		}

	}

	Ok(())
}