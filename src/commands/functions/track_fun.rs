use crate::{utils::embed::build_new_song_embed, AuxMetadataKey};
use serenity::{
    builder::CreateMessage, client::Context, framework::standard::Args, model::channel::Message,
};
use songbird::{
    input::{Compose, YoutubeDl},
    Call,
};
use tokio::sync::MutexGuard;
use tracing::warn;

pub async fn add_track_to_queue<'a>(
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

        //TrackQueue::new();

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
