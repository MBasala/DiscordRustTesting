use reqwest::json;
use std::collections::HashMap;
use url::{Url, ParseError};
use serenity::framework::standard::CommandResult;


struct webArchive_post
{
	url: String
}

#[command]
#[num_args(3)]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
pub async fn archive(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
	let archive_url = &args.single::<url>();

	let webArchive_post = newPost
	{
		url: archive_url,
	};

	let webArch_json: serde_json::Value = reqwest::Client::new()
		.post("https://web.archive.org/save")
		.json(&serde_json::json!
		({
			"ToDo" : "ToDo"
		}))
		.send().await?
		.json().await?;
}