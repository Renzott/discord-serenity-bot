use std::sync::Arc;

use crate::{
    check_msg,
    commands::{
        events::leave_channel::{ClientDisconnectNotifier, DriverDisconnect}, functions::track_fun::add_track_to_queue,
    },
    utils::common::get_http_client,
};
use serenity::{
    all::{ChannelId, Guild, Message, UserId, VoiceState},
    builder::CreateMessage,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};
use songbird::{CoreEvent, Event};
use tracing::{info, warn};

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
        let (new_guild, channel_id) = {
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

            let guild: Guild = guild.clone();
            (guild, channel_id)
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

        let new_guild_id = new_guild.id;

        if let Ok(handler_lock) = manager.join(new_guild_id, connect_to).await {
            let mut handler = handler_lock.lock().await;

            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Conectado al canal de voz")
                    .await,
            );

            let bot_id: UserId = ctx.cache.current_user().id;
            let context_arc = Arc::new(ctx.clone());

            handler.add_global_event(
                Event::Core(CoreEvent::ClientDisconnect),
                ClientDisconnectNotifier {
                    bot_id,
                    guild: new_guild.clone(),
                    http: ctx.http.clone(),
                    serenity_context: context_arc.clone(),
                    songbird_context: manager.clone(),
                    debounce: Default::default(),
                },
            );

            handler.add_global_event(
                Event::Core(CoreEvent::DriverDisconnect),
                DriverDisconnect {
                    bot_id,
                    guild: new_guild,
                    songbird_context: manager,
                    serenity_context: context_arc,
                },
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
