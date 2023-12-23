use std::time::Duration;

use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateMessage},
    client::Context,
    framework::standard::{macros::command, CommandResult, Args},
    model::{channel::Message, Colour},
};
use songbird::input::{Compose, YoutubeDl};

use crate::HttpKey;
use reqwest::Client;

#[command]
#[only_in(guilds)]
#[aliases("d", "dumm")]
async fn dummy(ctx: &Context, msg: &Message, args: Args) -> CommandResult {

    let input = args.message().to_string();

    let yt_url = match input.starts_with("http"){
        true => {
            let url = input.split_whitespace().next().unwrap();
            url.to_string()
        },
        false => {
            let format_url = format!("ytsearch:{}", input);
            format_url
        }
    };

    let http_client = get_http_client(&ctx).await;

    let mut yt_data = YoutubeDl::new(http_client, yt_url);

    let data_yt = yt_data.aux_metadata().await;

    if let Ok(aux_metadata) = data_yt {
        // aux_metadata.title is a Option<String>
        let author_embed = CreateEmbedAuthor::new("Nueva canción añadida ")
            .icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/6/6a/Youtube_Music_icon.svg/2048px-Youtube_Music_icon.svg.png");

        let duration = "`".to_string() + &duration_to_string(aux_metadata.duration.unwrap()) + "`";
        
        let fields = vec![
            ("Autor", aux_metadata.artist.unwrap(), true),
            ("Duración",duration,true,)
        ];
        
        let description = format!("[{}]({})", aux_metadata.title.unwrap(), aux_metadata.source_url.unwrap());

        let embed = CreateEmbed::new()
            .fields(fields)
            .author(author_embed)
            .thumbnail(aux_metadata.thumbnail.unwrap())
            .description(description)
            .color(Colour::new(0xFF0000));

        let builder = CreateMessage::new().embed(embed);

        let msg = msg.channel_id.send_message(&ctx.http, builder).await;

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }
    }

    Ok(())
}

async fn get_http_client(ctx: &Context) -> Client {
    let data = ctx.data.read().await;
    data.get::<HttpKey>()
        .cloned()
        .expect("Guaranteed to exist in the typemap.")
}

fn duration_to_string(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let seconds = seconds % 60;
    let minutes = minutes % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}
