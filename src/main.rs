use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serde_json::Value;
use std::collections::HashMap;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("bread") && !msg.author.bot {
            match get_bread_image().await {
                Some(url) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &url).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                None => println!("Error getting bread image."),
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "";

    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn get_bread_image() -> Option<String> {
    let random_page = rand::random::<u32>() % 1000;
    let url = format!("https://gelbooru.com/index.php?page=dapi&s=post&q=index&json=1&tags=bread&pid={}&limit=1", random_page);
    let resp = reqwest::get(&url).await.ok()?;
    let body = resp.text().await.ok()?;
    println!("Response body: {}", body);
    let data: HashMap<String, Value> = serde_json::from_str(&body).ok()?;
    let posts = data.get("post")?.as_array()?;
    let post = posts.get(0)?;
    post["file_url"].as_str().map(|s| s.to_string())
}