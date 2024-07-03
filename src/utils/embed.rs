use std::{io::Error, time::Duration};

use serenity::{
    all::UserId, builder::{CreateEmbed, CreateEmbedAuthor}, model::Colour
};
use songbird::input::AuxMetadata;
use tracing::warn;

use super::common::{
    duration_to_string, get_color_for_roll, get_dice_image_url, get_random_index_from_vec,
    get_random_number_from_range, get_welcome_farewell_data,
};

pub fn build_new_song_embed(song_metadata: AuxMetadata) -> CreateEmbed {
    let author_embed = CreateEmbedAuthor::new("Nueva canción añadida")
        .icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/6/6a/Youtube_Music_icon.svg/2048px-Youtube_Music_icon.svg.png");

    let duration = match song_metadata.duration {
        Some(d) => "`".to_string() + &duration_to_string(d) + "`",
        None => "🔴 En vivo".to_string(),
    };

    let fields = vec![
        ("Autor", song_metadata.artist.unwrap(), true),
        ("Duración", duration, true),
    ];

    let description = format!(
        "[{}]({})",
        song_metadata.title.unwrap(),
        song_metadata.source_url.unwrap()
    );

    let embed = CreateEmbed::new()
        .fields(fields)
        .author(author_embed)
        .thumbnail(song_metadata.thumbnail.unwrap())
        .description(description)
        .color(Colour::new(0xFF0000));

    embed
}

// build current song embed
pub fn build_current_song_embed(song_metadata: AuxMetadata, current_time: Duration) -> CreateEmbed {
    let author_embed = CreateEmbedAuthor::new("Reproduciendo ahora")
        .icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/6/6a/Youtube_Music_icon.svg/2048px-Youtube_Music_icon.svg.png");

    let duration = format!(
        "`{} / {}`",
        duration_to_string(current_time),
        duration_to_string(song_metadata.duration.unwrap())
    );

    let fields = vec![
        ("Autor", song_metadata.artist.unwrap(), true),
        ("Duración", duration, true),
    ];

    let description = format!(
        "[{}]({})",
        song_metadata.title.unwrap(),
        song_metadata.source_url.unwrap()
    );

    let embed = CreateEmbed::new()
        .fields(fields)
        .author(author_embed)
        .thumbnail(song_metadata.thumbnail.unwrap())
        .description(description)
        .color(Colour::new(0xFF0000));

    embed
}

pub fn build_new_birthay_embed(
    username: &String,
    image_url: String,
    message: String,
) -> CreateEmbed {
    let embed = CreateEmbed::new()
        //`🎉  Feliz Cum ${userData?.nickname || userData.user.username}  🎉`
        .title(format!("🎉  Feliz Cum {}  🎉", username))
        .description(message)
        .image(image_url)
        .color(Colour::new(0xf9f900));
    embed
}

pub async fn build_new_user_embed(username_id: UserId) -> Result<CreateEmbed, Error> {
    let data = match get_welcome_farewell_data().await {
        Ok(data) => data,
        Err(e) => {
            warn!("Error reading welcome farewell data: {:?}", e);
            return Err(e);
        }
    };

    let rand_dice = get_random_number_from_range(1, 20);

    let dice_image_url = get_dice_image_url(rand_dice);

    let welcome_title = data
        .welcome_messages
        .get(get_random_index_from_vec(&data.welcome_messages))
        .unwrap()
        .to_string();

    let message = match rand_dice {
        1 => data
            .dices
            .mistake
            .get(get_random_index_from_vec(&data.dices.mistake))
            .unwrap(),
        2..=5 => data
            .dices
            .very_low
            .get(get_random_index_from_vec(&data.dices.very_low))
            .unwrap(),
        6..=9 => data
            .dices
            .low
            .get(get_random_index_from_vec(&data.dices.low))
            .unwrap(),
        10..=11 => data
            .dices
            .medium_low
            .get(get_random_index_from_vec(&data.dices.medium_low))
            .unwrap(),
        12..=14 => data
            .dices
            .medium
            .get(get_random_index_from_vec(&data.dices.medium))
            .unwrap(),
        15..=16 => data
            .dices
            .medium_high
            .get(get_random_index_from_vec(&data.dices.medium_high))
            .unwrap(),
        17..=18 => data
            .dices
            .high
            .get(get_random_index_from_vec(&data.dices.high))
            .unwrap(),
        19 => data
            .dices
            .very_high
            .get(get_random_index_from_vec(&data.dices.very_high))
            .unwrap(),
        20 => data
            .dices
            .critical
            .get(get_random_index_from_vec(&data.dices.critical))
            .unwrap(),
        _ => {
            warn!("Error getting dice data");
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Error getting dice data",
            ));
        }
    };

    let message = format!("<@{}> _{}_", username_id, message);
    
    let embed_color = get_color_for_roll(rand_dice);

    let embed = CreateEmbed::new()
        .title(welcome_title)
        .description(message)
        .thumbnail(dice_image_url)
        .color(Colour::from(embed_color));

    Ok(embed)
}
