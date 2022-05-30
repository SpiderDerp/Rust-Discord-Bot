use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::Args;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use serenity::{
    async_trait,
    builder::CreateMessage,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::channel::Embed;
use serenity::utils::Colour;
use serenity::builder::CreateEmbed;


#[group]
#[commands()] //Put command names into parenthesis
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("")) // set the bot's prefix
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = "";
    let mut client = Client::builder(token, Handler)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}


