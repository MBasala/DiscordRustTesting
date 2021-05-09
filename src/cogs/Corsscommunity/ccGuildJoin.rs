use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};


#[path = "../../sql/mod.rs"]
mod sql;


//ToDo: commit a transaction to the sql Cross_Community_Guild_Join table and update the server with the CC information.
#[command]
#[num_args(3)]
#[only_in("guilds")]
#[required_permissions("BAN_MEMBER")]
pub async fn ccBan(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{

	match (&args).single::<id::UserId>() {
		Err(_) => {
				msg.channel_id
					.say(&ctx, "Not a valid Member ID.")
					.await?;
		}
		Ok(mem) => match (&args).single::<Strings>() {
			Err(_) => {
					msg.channel_id
						.say(&ctx, "You need a valid reason for banning.")
						.await?;
				}
			Ok(res) => match msg.guild_id {
				None => {
						msg.channel_id
							.say(&ctx, "You need a valid server to ban a user.")
							.await?;
					}
				Some(g) => {
						if false {
							match push_guild_cc(create_connection("testing")) {

								Err(e) => {
									msg.channel_id
										.say(&ctx, format!("Error connecting to the SQL server with error: {}.", e))
										.await?;
								}
								Ok(conn) => {
									match push_guild_cc(g) {
										Ok(_) => {
											let member = g.member(&ctx, mem)
												.ban_with_reason(&ctx, 0, &res)
												.await?;

											msg.channel_id
												.say(&ctx, format!("Successfully Banned {}, for: {}", member.user.name, &res))
												.await?;
										}
										Err(e) => {
											msg.channel_id
												.say(&ctx, format!("Error pushing new guild cross community relationship with error: {}", e))
												.await?;
										}
									}
								}
							}
						}
					}
			}
		}

	}

	Ok(())
}
