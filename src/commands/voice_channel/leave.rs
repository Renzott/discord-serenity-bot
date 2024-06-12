use crate::check_msg;

use serenity::all::{
    standard::macros::command,
    standard::CommandResult,
    client::Context,
};
use serenity::model::channel::Message;

#[command]
#[only_in(guilds)]
#[aliases("leave")]
async fn leave_channel(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }
        // Left voice channel in spanish
        check_msg(msg.channel_id.say(&ctx.http, "El bot ha salido del canal de voz").await);
    } else {
        check_msg(msg.reply(ctx, "El bot no está en un canal de voz").await);
    }

    Ok(())
}