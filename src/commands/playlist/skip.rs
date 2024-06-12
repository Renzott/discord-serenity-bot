use crate::{check_msg, AuxMetadataKey};
use serenity::{
    all::Message,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

#[command]
#[only_in(guilds)]
#[aliases("skip")]
async fn skip_song(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();

        let title = match queue.current() {
            Some(track) => {
                let typemap = track.typemap().read().await;
                typemap.get::<AuxMetadataKey>().unwrap().title.clone()
            }
            None => None,
        };

        if let Some(title) = title {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Se ha saltado la canción: {}", title))
                    .await,
            );
        } else if queue.len() == 0 {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "No hay canciones en la cola")
                    .await,
            );
        } else {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Hay un error en la información de la canción")
                    .await,
            );
        }
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "No estoy en un canal de voz")
                .await,
        );
    }
    Ok(())
}