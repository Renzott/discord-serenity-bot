use std::time::Duration;

use crate::{
    check_msg,
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
use songbird::{
    input::{Compose, YoutubeDl},
    Call,
};
use tokio::sync::MutexGuard;
use tracing::warn;

#[command]
#[only_in(guilds)]
#[aliases("p", "play")]
async fn play_song(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let http_client = get_http_client(ctx).await;

    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        

        let _ = add_track_to_queue(msg, args, handler, ctx, http_client).await;
    } else {
        // if the bot is not connected to a voice channel, connect to the user's voice channel
        let (new_guild_id, channel_id) = {
            let guild = msg.guild(&ctx.cache).unwrap();
            let channel_id = guild
                .voice_states
                .get(&msg.author.id)
                .and_then(|voice_state| voice_state.channel_id);
            (guild.id, channel_id)
        };

        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                let builder = CreateMessage::new().content("No estás en un canal de voz");
                let msg = msg.channel_id.send_message(&ctx.http, builder).await;

                if let Err(why) = msg {
                    warn!("Error sending message: {:?}", why);
                }
                return Ok(());
            }
        };

        if let Ok(handler_lock) = manager.join(new_guild_id, connect_to).await {
            let handler = handler_lock.lock().await;

            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Conectado al canal de voz")
                    .await,
            );

            let _ = add_track_to_queue(msg, args, handler, ctx, http_client).await;
        } else {
            let builder = CreateMessage::new().content("Error al unirse al canal de voz");
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                warn!("Error sending message: {:?}", why);
            }
            return Ok(());
        }
    }
    Ok(())
}

async fn add_track_to_queue<'a>(
    msg: &Message,
    args: Args,
    mut handler: MutexGuard<'a, Call>,
    ctx: &'a Context,
    http_client: reqwest::Client,
) -> Result<(), ()> {
    let input = args.message().to_string();

    if input.contains("playlist") && input.starts_with("http") {
        let builder =
            CreateMessage::new().content("No se pueden reproducir playlists, por ahora (wip)");
        let msg = msg.channel_id.send_message(&ctx.http, builder).await;

        if let Err(why) = msg {
            warn!("Error sending message: {:?}", why);
        }
        return Ok(());
    }

    let yt_query = match input.starts_with("http") {
        true => {
            let url = input.split_whitespace().next().unwrap();
            url.to_string()
        }
        false => {
            let format_url = format!("ytsearch:{}", input);
            format_url
        }
    };

    let yt_data = YoutubeDl::new(http_client, yt_query);

    let build_aux_metadata = yt_data.clone().aux_metadata().await;

    if let Ok(aux_metadata) = build_aux_metadata {
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
    } else {
        warn!("Error getting metadata from track");
        msg.channel_id
            .say(&ctx.http, "Error al obtener metadatos de la canción")
            .await
            .unwrap();
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


//command to list of the songs in the queue
#[command]
#[only_in(guilds)]
#[aliases("q", "queue","list","songs")]
async fn list_of_all_songs(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx).await.unwrap().clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue().current_queue();

        if queue.is_empty() {
            let builder = CreateMessage::new().content("No hay canciones en la cola");
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                warn!("Error sending message: {:?}", why);
            }
        } else {
            let mut queue_list = String::new();
            for (index, track) in queue.iter().enumerate() {
                let typemap = track.typemap().read().await;
                let aux_metadata = typemap.get::<AuxMetadataKey>();

                if let Some(aux_metadata) = aux_metadata {
                    let aux_metadata = aux_metadata.clone();
                    queue_list.push_str(&format!("{}. {}\n", index + 1, aux_metadata.title.unwrap_or_default()));
                }
            }

            let builder = CreateMessage::new().content(queue_list);
            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                warn!("Error sending message: {:?}", why);
            }
        }
    }
    Ok(())
}