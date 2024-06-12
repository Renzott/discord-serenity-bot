use crate::check_msg;
use serenity::{
    all::Message,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[only_in(guilds)]
#[aliases("resume")]
async fn resume_song(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.resume();

        check_msg(msg.channel_id.say(&ctx.http, "Se ha reanudado la música").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}