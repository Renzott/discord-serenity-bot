use std::time::Duration;

use crate::{utils::embed::build_current_song_embed, AuxMetadataKey};
use serenity::{
    builder::CreateMessage,
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};
use tracing::warn;

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
