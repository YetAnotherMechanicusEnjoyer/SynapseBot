use crate::load_config;
use poise::serenity_prelude::Message;
use redis::{AsyncCommands, Client, RedisResult};

pub async fn message_create(msg: Message) {
    let prefix = load_config("./config.json")
        .expect("Failed to load config.json")
        .discord_prefix;

    if msg.author.bot || msg.content.starts_with(&prefix) {
        return;
    }

    let channel = std::env::var("DISCORD_CHANNEL_ID")
        .expect("Expected a TWITCH_BOT_USERNAME environment variable.");

    if msg.channel_id == channel.parse::<u64>().unwrap() {
        let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://redis:6379".into());

        let client = match Client::open(redis_url) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to connect to Redis: {e}");
                return;
            }
        };

        let mut con = match client.get_multiplexed_async_connection().await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to get Redis connection: {e}");
                return;
            }
        };

        let redis_channel_name = "discord_messages";
        let message_to_publish = format!("{}|{}", msg.content, msg.author.name);

        let result: RedisResult<()> = con.publish(redis_channel_name, &message_to_publish).await;

        if let Err(e) = result {
            eprintln!("Failed to publish message to Redis: {e}");
        } else {
            println!("Successfully published message to Redis channel '{redis_channel_name}'",);
        }
    }
}
