use serenity::all::{
    standard::macros::command,
    standard::CommandResult,
    client::Context,
};

use serenity::model::channel::Message;
use songbird::tracks::LoopState;

use crate::check_msg;


#[command]
#[only_in(guilds)]
#[aliases("loop","repeat")]
async fn loop_song(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.");

    let call = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            check_msg(msg.reply(ctx, "El bot no está en un canal de voz").await);
            return Ok(());
        }
    };

    let handler = &call.lock().await;

    let current_track = match handler.queue().current() {
        Some(song) => song,
        None => {
            check_msg(msg.reply(ctx, "No hay canciones en la cola").await);
            return Ok(());
        }
    };

    let is_song_looping = match current_track.get_info().await {
        Ok(info) => info.loops == LoopState::Infinite,
        Err(why) => {
            check_msg(msg.reply(ctx, &format!("Error obteniendo información de la canción: {:?}", why)).await);
            return Ok(());
        }
    };

    if is_song_looping {
        let _ = current_track.disable_loop();
        check_msg(msg.reply(ctx, "La canción ya no se repetirá indefinidamente").await);
    } else {
        let _ = current_track.enable_loop();
        check_msg(msg.reply(ctx, "La canción se repetirá indefinidamente").await);
    }

    Ok(())
}