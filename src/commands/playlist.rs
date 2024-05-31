use std::time::Duration;

use crate::{
    utils::{
        common::get_http_client,
        embed::{build_current_song_embed, build_new_song_embed},
    },
    AuxMetadataKey,
};
use serenity::{
    builder::CreateMessage,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};
use songbird::input::{Compose, YoutubeDl};
use tracing::warn;

#[command]
#[only_in(guilds)]
#[aliases("p", "play", "queue")]
async fn play_song(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    
    let input = args.message().to_string();

    if input.contains("playlist") && input.starts_with("http") {
        let builder = CreateMessage::new().content("No se pueden reproducir playlists, por ahora (wip)");
        let msg = msg.channel_id.send_message(&ctx.http, builder).await;

        if let Err(why) = msg {
            warn!("Error sending message: {:?}", why);
        }
        return Ok(());
    }

    let yt_url = match input.starts_with("http") {
        true => {
            let url = input.split_whitespace().next().unwrap();
            url.to_string()
        }
        false => {
            let format_url = format!("ytsearch:{}", input);
            format_url
        }
    };

    let guild_id = msg.guild_id.unwrap();
    let http_client = get_http_client(ctx).await;

    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let yt_data = YoutubeDl::new(http_client, yt_url);

        let builder_embed = yt_data.clone().aux_metadata().await;

        if let Ok(aux_metadata) = builder_embed {
            let embed = build_new_song_embed(aux_metadata.clone());

            let builder = CreateMessage::new().embed(embed);

            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                warn!("Error sending message: {:?}", why);
            }

            let track = handler.enqueue_input(yt_data.into()).await;
            track
                .typemap()
                .write()
                .await
                .insert::<AuxMetadataKey>(aux_metadata);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases("nowplaying", "current", "np")]
async fn now_playing(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let current = handler.queue().current();

        if let Some(track) = current {

            let time = match track.get_info().await {
                Ok(info) => info.position,
                Err(why) => {
                    warn!("Error getting track info: {:?}", why);
                    Duration::from_secs(0)
                }
            };

            let typemap = track.typemap().read().await;
            let aux_metadata = typemap.get::<AuxMetadataKey>();

            if let Some(aux_metadata) = aux_metadata {
                let embed = build_current_song_embed(aux_metadata.clone(), time);

                let builder = CreateMessage::new().embed(embed);

                let msg = msg.channel_id.send_message(&ctx.http, builder).await;

                if let Err(why) = msg {
                    warn!("Error sending message: {:?}", why);
                }
            } else {
                warn!("Error getting metadata from track")
            }
        } else {
            let builder =
                CreateMessage::new().content("No hay canciones reproduciendo en este momento");
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                warn!("Error sending message: {:?}", why);
            }
        }
    }
    Ok(())
}
