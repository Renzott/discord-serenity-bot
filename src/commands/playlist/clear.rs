use crate::check_msg;
use serenity::{
    all::Message,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[only_in(guilds)]
#[aliases("clear", "stopall")]
async fn stop_and_clear(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.stop();

        check_msg(msg.channel_id.say(&ctx.http, "Playlist limpiada").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "No estoy en un canal de voz")
                .await,
        );
    }

    Ok(())
}