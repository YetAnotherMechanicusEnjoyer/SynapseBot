use poise::serenity_prelude::{self as serenity, EventHandler, UserId};
use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path, sync::Arc};
use tokio::sync::Mutex;

mod cmds;
mod events;

#[derive(Debug, Clone)]
struct GameState {
    player1_id: UserId,
    player2_id: UserId,
    player1_hp: u32,
    player2_hp: u32,
    turn: UserId,
}

pub struct Data {
    color: (u8, u8, u8),
    active_duels: Arc<Mutex<HashMap<serenity::MessageId, GameState>>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Deserialize, Debug)]
pub struct Config {
    discord_prefix: String,
    color: [u8; 3],
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&file_contents)?;
    Ok(config)
}

struct Handler;

#[poise::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: serenity::Context, _data_about_bot: serenity::Ready) {
        println!("Bot is ready!");
    }

    async fn message(&self, ctx: serenity::Context, new_message: serenity::Message) {
        if new_message.author.id != ctx.cache.current_user().id {
            events::discord::message_create::message_create(new_message).await;
        }
    }
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::InteractionCreate { interaction } = event {
        events::discord::interaction_create::interaction_create(
            ctx,
            interaction.clone(),
            framework,
            _data,
        )
        .await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    println!("Loading discord bot...");

    let discord_token =
        env::var("DISCORD_BOT_TOKEN").expect("Expected a DISCORD_BOT_TOKEN environment variable.");

    let commands = cmds::get_all_commands();

    let config = load_config("./config.json").expect("Failed to load config.json");

    let color = (config.color[0], config.color[1], config.color[2]);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.discord_prefix),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    color,
                    active_duels: Arc::new(Mutex::new(HashMap::new())),
                })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MESSAGES;

    let discord_client = serenity::ClientBuilder::new(&discord_token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await;

    let redis_url = std::env::var("REDIS_URL").unwrap_or("redis://redis:6379".into());

    tokio::spawn(async move {
        if let Err(e) =
            events::twitch::start_redis_listener(serenity::Http::new(&discord_token), &redis_url)
                .await
        {
            eprintln!("Redis listener error: {e}");
        }
    });

    if let Err(why) = discord_client.unwrap().start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
