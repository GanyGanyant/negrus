use super::*;

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );
            return Ok(());
        }
    };

    if !url.starts_with("http") {
        url = match video::search(args.message()).await {
            Ok(url) => url,
            Err(e) => {
                check_msg(msg.channel_id.say(&ctx.http, e).await);
                return Ok(());
            }
        }
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = || async {
        return songbird::get(ctx)
            .await
            .expect("Songbird Voice client placed in at initialisation.")
            .clone();
    };

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let Some(channel) = channel_id else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
        return Ok(());
    };

    if manager().await.get(guild_id).is_none() {
        let (_,Ok(_)) = manager().await.join(guild_id, channel).await else {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Error joining channel")
                    .await,
            );
            return Ok(())
        };
    }

    let err = || async {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "You must be in the same voice channel as bot.")
                .await,
        );
    };

    if let Some(handler_lock) = manager().await.get(guild_id) {
        let Some(bot_channel) = handler_lock.lock().await.current_channel() else {
            err().await;
            return Ok(());
        };

        if let Some(channel) = channel_id {
            if songbird::id::ChannelId::from(channel) != bot_channel {
                let Ok(y) = channel.to_channel(ctx).await else {
                    err().await;
                    return Ok(());
                };
                let Some(x) = y.guild() else {
                    err().await;
                    return Ok(());
                };
                let Some(z) = x.member_count else {
                    err().await;
                    return Ok(());
                };
                if z > 1u8 {
                    err().await;
                    return Ok(());
                }
                let (_,Ok(_)) = manager().await.join(guild_id, channel).await else {
                    check_msg(
                        msg.channel_id
                            .say(&ctx.http, "Error joining channel")
                            .await,
                    );
                    return Ok(())
                };
            }
        }

        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };

        handler.play_source(source);

        check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}
