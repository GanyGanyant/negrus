use std::env;

use rand::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::SipHasher;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::model::prelude::UserId;
use serenity::prelude::*;

use songbird::SerenityInit;

mod music;
use music::*;

#[group]
#[commands(
    help, negr, pp, rand, gay, deafen, join, leave, mute, play, ping, undeafen, unmute
)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let ans = format!("!help\n!negr\n!rand <bool>(true/false)/<1>(0.0-1.0)/<n>(0-n)\n!pp\n!gay\n!join\n!leave\n!mute\n!play\n!ping\n!undeafen\n!unmute");
    msg.reply(ctx, ans).await?;

    Ok(())
}

#[command]
async fn negr(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "černý!").await?;

    Ok(())
}

#[command]
async fn pp(ctx: &Context, msg: &Message) -> CommandResult {
    let id = msg.author.id;
    let gany = UserId::from(395907583614910465);

    // If we want to be more explicit, first we create a SipRng:
    let hasher = SipHasher::from(id);
    let mut hasher_rng = hasher.into_rng();
    // (Note: hasher_rng is a full RNG and can be used directly.)

    // Now, we use hasher_rng to create a seed:
    let mut seed: <Pcg64 as SeedableRng>::Seed = Default::default();
    hasher_rng.fill(&mut seed);

    // And create our RNG from that seed:
    let mut rng = Pcg64::from_seed(seed);
    let mut size = rng.gen_range(1..26);

    if id == gany {
        size = 17;
    }

    let pp = std::iter::repeat("=").take(size).collect::<String>();
    let ans = format!("Your pp is {}cm long\n8{}D", size, pp);
    msg.reply(ctx, ans).await?;

    Ok(())
}

#[command]
async fn rand(ctx: &Context, msg: &Message) -> CommandResult {
    let input = msg.content.to_string();
    let split: Vec<&str> = input.split_whitespace().collect();
    if let Some(n) = split.get(1) {
        let mut ans: String = format!("{} is not a valid parameter.", n);
        if n.to_uppercase() == "BOOL".to_string() {
            let b: bool = rand::thread_rng().gen();
            ans = b.to_string();
        } else if let Ok(n) = n.parse::<u64>() {
            if n == 1 {
                let f: f64 = rand::thread_rng().gen();
                ans = f.to_string();
            } else {
                let i = rand::thread_rng().gen_range(0..=n);
                ans = i.to_string();
            }
        }
        msg.reply(ctx, ans).await?;
    }
    Ok(())
}

#[command]
async fn gay(ctx: &Context, msg: &Message) -> CommandResult {
    let id = msg.author.id;
    //let gany = UserId::from(395907583614910465);

    // If we want to be more explicit, first we create a SipRng:
    let hasher = SipHasher::from(id);
    let mut hasher_rng = hasher.into_rng();
    // (Note: hasher_rng is a full RNG and can be used directly.)

    // Now, we use hasher_rng to create a seed:
    let mut seed: <Pcg64 as SeedableRng>::Seed = Default::default();
    hasher_rng.fill(&mut seed);

    // And create our RNG from that seed:
    let mut rng = Pcg64::from_seed(seed);
    let size = rng.gen_range(0..=100);
    //if id == gany {
    //    size = 18;
    //}
    let ans = format!("You are {}% gay", size);
    msg.reply(ctx, ans).await?;
    Ok(())
}
