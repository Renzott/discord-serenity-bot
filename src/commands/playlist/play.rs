use crate::{
    check_msg, commands::functions::track_fun::add_track_to_queue, utils::common::get_http_client,
};
use serenity::{
    all::{ChannelId, GuildId, Message, VoiceState},
    builder::CreateMessage,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};
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
        let (new_guild_id, channel_id) = {
            let guild = match msg.guild(&ctx.cache) {
                Some(guild) => guild,
                None => {
                    return Ok(());
                }
            };

            let channel_id: Option<ChannelId> = guild
                .voice_states
                .get(&msg.author.id)
                .and_then(|voice_state: &VoiceState| voice_state.channel_id);

            let guild_id: GuildId = guild.id;

            (guild_id, channel_id)
        };

        let connect_to = if let Some(channel) = channel_id {
            channel
        } else {
            let builder = CreateMessage::new().content("No estás en un canal de voz");

            match msg.channel_id.send_message(&ctx.http, builder).await {
                Ok(_) => (),
                Err(why) => warn!("Error sending message: {:?}", why),
            }

            return Ok(());
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
