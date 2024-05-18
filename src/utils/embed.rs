use serenity::{builder::{ CreateEmbed, CreateEmbedAuthor}, model::Colour};
use songbird::input::AuxMetadata;

use super::common::duration_to_string;

pub fn build_new_song_embed(song_metadata: AuxMetadata) -> CreateEmbed {
    let author_embed = CreateEmbedAuthor::new("Nueva canción añadida ")
        .icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/6/6a/Youtube_Music_icon.svg/2048px-Youtube_Music_icon.svg.png");

    let duration = "`".to_string() + &duration_to_string(song_metadata.duration.unwrap()) + "`";
    
    let fields = vec![
        ("Autor", song_metadata.artist.unwrap(), true),
        ("Duración",duration,true,)
    ];
    
    let description = format!("[{}]({})", song_metadata.title.unwrap(), song_metadata.source_url.unwrap());

    let embed = CreateEmbed::new()
        .fields(fields)
        .author(author_embed)
        .thumbnail(song_metadata.thumbnail.unwrap())
        .description(description)
        .color(Colour::new(0xFF0000));

    embed
}

pub fn build_new_birthay_embed(username: &String, image_url: String, message: String ) -> CreateEmbed {
    let embed = CreateEmbed::new()
        //`🎉  Feliz Cum ${userData?.nickname || userData.user.username}  🎉`
        .title(format!("🎉  Feliz Cum {}  🎉", username))
        .description(message)
        .image(image_url)
        .color(Colour::new(0xf9f900));
    embed
}