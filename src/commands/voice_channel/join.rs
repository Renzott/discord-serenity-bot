use std::sync::Arc;

use crate::check_msg;
use crate::commands::events::leave_channel::ClientDisconnectNotifier;

use serenity::all::UserId;
use serenity::all::{client::Context, standard::macros::command, standard::CommandResult};
use serenity::{model::channel::Message, prelude::Mentionable};
use songbird::{CoreEvent, Event};

#[command]
#[only_in(guilds)]
#[aliases("join")]
async fn join_channel(ctx: &Context, msg: &Message) -> CommandResult {
    let (guild_id, channel_id) = {
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
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Ok(_handle_lock) = manager.join(guild_id, connect_to).await {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

        let mut handler = _handle_lock.lock().await;

        let current_guild = msg.guild(&ctx.cache).unwrap();

        let bot_id: UserId = ctx.cache.current_user().id;

        let context_arc = Arc::new(ctx.clone());

        handler.add_global_event(
            Event::Core(CoreEvent::ClientDisconnect),
            ClientDisconnectNotifier{
                bot_id,
                guild: current_guild.clone(),
                http: ctx.http.clone(),
                serenity_context: context_arc,
                songbird_context: manager.clone(),
                debounce: Default::default(),
            }
        );

    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }

    Ok(())
}
