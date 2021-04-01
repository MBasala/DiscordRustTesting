

use std::{collections::{HashMap, HashSet}, env, fmt::Write, sync::Arc};
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    collector::MessageCollectorBuilder,
    framework::standard::{
        Args, CommandOptions, CommandResult, CommandGroup,  Configuration,
        DispatchError, HelpOptions, help_commands, Reason, StandardFramework,
        buckets::{RevertBucket, LimitedFor},
        macros::{command, group, help, check, hook},
    },
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
    utils::{content_safe, ContentSafeOptions},
};

mod cogs;

use std::arch::x86_64::_mm_sha1msg1_epu32;
use serenity::prelude::*;
use tokio::sync::{Mutex, RwLockWriteGuard};

use cogs::{
    ping::*,
    ban::*,
};

struct Commands;

#[group]
#[commands(ping, ban)]
struct General;

#[hook]
async fn before(ctx: &Context, msg: &Message, command_name: &str) -> bool {
    println!("Got command '{}' by user '{}'", command_name, msg.author.name);

    // Increment the number of times this command has been run once. If
    // the command's name does not exist in the counter, add a default
    // value of 0.
    let mut data: RwLockWriteGuard<TypeMap> = ctx.data.write().await;
    let counter = data.get_mut::<CommandCounter>().expect("Expected CommandCounter in TypeMap.");
    let entry = counter.entry(command_name.to_string()).or_insert(0);
    *entry += 1;

    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(()) => println!("Processed command '{}'", command_name),
        Err(why) => println!("Command '{}' returned error {:?}", command_name, why),
    }
}

#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}

fn main()
{
    println!("Booting");

    let token= env::var("Testing")
        .expect("Expected a token in the environment");

    let framework :StandardFramework = StandardFramework::new()
        .configure(|c  | c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("~")
            .delimiters(vec![", ", ","]))
        .help( &MY_HELP)
        .group(&GENERAL_GROUP);

    let mut client :Client = Client::builder(&token)
        .event_handler(Commands)
        .framework(framework)
        .await
        .expect("Error building client");
}

/*#[async_Trait]
impl EventHandler for Commands
{
    async fn message(&self, ctx: Context, msg: Message)
    {
        if msg.content == "!testing"
        {
            let channel :Channel = match msg.channel_id.to_channel(&Context).await
            {
                Ok(channel ) => channel,
                Err(why) =>
                    {
                        println!("Error sending output: {:?}", why);
                        return;
                    }
            };


            let response :string = MessageBuilder::new()
                .push("User ") :&mut MessageBuilder
                .push_bold_safe(&msg.author.name) :&mut MessageBuilder
                .push(" has tested this command in the channel") :&mut MessageBuilder
                .mention(&channel) :&mut MessageBuilder
                .push(" ") :&mut MessageBuilder
                .build();

            if let Err(why) = msg.channel_id.say(&Context.http, &response).await
            {
                print!("Error sending message: {:?}", why);
            }
        }

        async fn ready(&self, _: Context, read: Ready)
        {
            print!("{} is connected!", ready.user.name);
        }

    }
}*/