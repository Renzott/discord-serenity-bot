use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message, builder::CreateMessage,
};
use songbird::input::{YoutubeDl, Compose};
use tracing::warn;
use crate::utils::{embed::build_new_song_embed, common::get_http_client};

#[command]
#[only_in(guilds)]
#[aliases("p", "play", "queue")]
async fn play_song(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let input = args.message().to_string();

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
            let embed = build_new_song_embed(aux_metadata);

            let builder = CreateMessage::new().embed(embed);

            let msg = msg.channel_id.send_message(&ctx.http, builder).await;

            if let Err(why) = msg {
                warn!("Error sending message: {:?}", why);
            }

            handler.enqueue_input(yt_data.into()).await;
        }
    }
    Ok(())
}


