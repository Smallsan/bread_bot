use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use reqwest::Error;
use serde_json::Value;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("bread") && !msg.author.bot {
            match get_bread_image().await {
                Ok(url) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &url).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                Err(why) => println!("Error getting bread image: {:?}", why),
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "your bot token here";

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn get_bread_image() -> Result<String, Error> {
    let resp = reqwest::get("https://danbooru.donmai.us/posts.json?tags=bread&limit=100").await?;
    let posts: Value = resp.json().await?;
    let post = &posts[rand::random::<usize>() % posts.as_array().unwrap().len()];
    Ok(post["file_url"].as_str().unwrap().to_string())
}
