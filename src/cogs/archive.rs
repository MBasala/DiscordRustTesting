use reqwest::{json, header, Url};
use std::collections::HashMap;
use url::{Url, ParseError};
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
	Args, CommandResult,
	macros::command,
};

//ToDo: create struct that reflects an web archive http request.
struct WebarchivePost
{
	url :Url,
	params :T,
	header :header::USER_AGENT,
	backoff_facto: u8,
	no_raise_on_redirects: bool,
}

#[command]
#[num_args(3)]
#[only_in(guilds)]
#[required_permissions("BAN_MEMBERS")]
pub async fn archive(ctx: &Context, msg: &Message, args: Args) -> CommandResult
{
	let archive_url = &args.single::<Url>();

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